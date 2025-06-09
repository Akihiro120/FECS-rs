use crate::types::{get_entity_index, Entity, NPOS, SPARSE_SET_SIZE};

pub struct SparseSet<T>
{
    dense: Vec<T>,
    dense_entities: Vec<Entity>,
    sparse: Vec<Box<[u32; SPARSE_SET_SIZE]>>,
}

impl<T> SparseSet<T>
{
    pub fn new() -> Self
    {
        SparseSet {
            dense: Vec::new(),
            dense_entities: Vec::new(),
            sparse: Vec::new(),
        }
    }

    pub fn get_sparse(&self) -> &Vec<Box<[u32; SPARSE_SET_SIZE]>>
    {
        &self.sparse
    }

    pub fn insert(&mut self, e: Entity, component: T)
    {
        let idx = get_entity_index(e);

        let dense_length = self.dense.len();

        let slot_position = {
            let slot = self
                .sparse_slot_mut(idx)
                .expect("Sparse position doesnt exist.");

            let current = *slot;

            if current == NPOS
            {
                *slot = dense_length as u32;
                *slot
            }
            else
            {
                current
            }
        };

        if slot_position as usize == dense_length
        {
            self.dense_entities.push(e);
            self.dense.push(component);
        }
        else
        {
            self.dense[slot_position as usize] = component;
        }
    }

    pub fn remove(&mut self, e: Entity)
    {
        let idx = get_entity_index(e);
        let last = match self.dense.len().checked_sub(1)
        {
            Some(x) => x,
            None =>
            {
                return;
            }
        };

        {
            let slot = match self.sparse_slot(idx).copied()
            {
                Some(s) if s != NPOS => s,
                _ => return,
            };

            if slot as usize != last
            {
                self.dense.swap(slot as usize, last);
                self.dense_entities.swap(slot as usize, last);

                let entity_index = self.dense_entities[slot as usize];
                let slot_at_entity = self.sparse_slot_mut(entity_index).unwrap();
                *slot_at_entity = slot;
            }
        }

        self.dense.pop();
        self.dense_entities.pop();

        let slot = self.sparse_slot_mut(idx).unwrap();
        *slot = NPOS;
    }

    pub fn has(&self, e: Entity) -> bool
    {
        let idx = get_entity_index(e);
        let offset = self.get_page_offset(idx) as usize;

        self.page_for(idx)
            .map(|page| page[offset] != NPOS)
            .unwrap_or(false)
    }

    pub fn get(&self, e: Entity) -> Option<&T>
    {
        let idx = get_entity_index(e);
        let offset = self.get_page_offset(idx) as usize;

        let page_position = {
            let page = self.page_for(idx).expect("Invalid Page");
            let page_offset = page[offset];

            if page_offset == NPOS
            {
                return None;
            }

            page_offset
        };

        self.dense.get(page_position as usize)
    }

    pub fn get_mut(&mut self, e: Entity) -> Option<&mut T>
    {
        let idx = get_entity_index(e);
        let offset = self.get_page_offset(idx) as usize;

        let page_position = {
            let page = self.page_for(idx).expect("Invalid Page");
            let page_offset = page[offset];

            if page_offset == NPOS
            {
                return None;
            }

            page_offset
        };

        self.dense.get_mut(page_position as usize)
    }

    pub fn size(&self) -> usize
    {
        self.dense.len()
    }

    pub fn entity_at(&self, index: u32) -> Entity
    {
        self.dense_entities
            .get(index as usize)
            .copied()
            .expect("No entity exists at index")
    }

    pub fn reserve(&mut self, amount: usize)
    {
        let num_pages = (amount + SPARSE_SET_SIZE - 1) / SPARSE_SET_SIZE;

        if num_pages > self.sparse.len()
        {
            self.sparse
                .resize(num_pages, Box::new([NPOS; SPARSE_SET_SIZE]));
        }

        self.dense.reserve(amount);
        self.dense_entities.reserve(amount);
    }

    pub fn clear(&mut self)
    {
        for page in &mut self.sparse
        {
            page.fill(NPOS);
        }

        self.dense.clear();
        self.dense_entities.clear();
    }

    fn sparse_slot(&mut self, idx: u32) -> Option<&u32>
    {
        let p: u32 = self.get_page_index(idx);

        if p >= self.sparse.len() as u32
        {
            self.sparse
                .resize(p as usize + 1, Box::new([NPOS; SPARSE_SET_SIZE]));
        }

        let page_index = self.get_page_offset(idx) as usize;
        self.sparse
            .get(p as usize)
            .map(|page| &**page)
            .and_then(|page| page.get(page_index))
    }

    fn sparse_slot_mut(&mut self, idx: u32) -> Option<&mut u32>
    {
        let p: u32 = self.get_page_index(idx);

        if p >= self.sparse.len() as u32
        {
            self.sparse
                .resize(p as usize + 1, Box::new([NPOS; SPARSE_SET_SIZE]));
        }

        let page_index = self.get_page_offset(idx) as usize;
        self.sparse
            .get_mut(p as usize)
            .map(|page| &mut **page)
            .and_then(|page| page.get_mut(page_index))
    }

    fn get_page_index(&self, idx: u32) -> u32
    {
        idx / SPARSE_SET_SIZE as u32
    }

    fn get_page_offset(&self, idx: u32) -> u32
    {
        idx % SPARSE_SET_SIZE as u32
    }

    // fn page_for_mut(&mut self, idx: u32) -> Option<&mut [u32; SPARSE_SET_SIZE]>
    // {
    //     let p: u32 = self.get_page_index(idx);
    //     self.sparse.get_mut(p as usize).map(|page| &mut **page)
    // }

    fn page_for(&self, idx: u32) -> Option<&[u32; SPARSE_SET_SIZE]>
    {
        let p: u32 = self.get_page_index(idx);
        self.sparse.get(p as usize).map(|page| &**page)
    }
}
