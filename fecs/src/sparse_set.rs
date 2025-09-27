use crate::component::Component;

const NPOS: usize = usize::MAX;

pub struct SparseSet<T>
{
    sparse: Vec<usize>,
    dense: Vec<T>,
    dense_entities: Vec<usize>,
}

impl<T> SparseSet<T>
where
    T: Component,
{
    pub fn new() -> Self
    {
        Self {
            sparse: Vec::new(),
            dense: Vec::new(),
            dense_entities: Vec::new(),
        }
    }

    pub fn insert(&mut self, id: usize, component: T) -> Result<(), ()>
    {
        self.sparse.resize(id + 1, NPOS);

        let sparse_idx = self.sparse[id];
        if sparse_idx != NPOS
        {
            // exists
            self.dense[sparse_idx] = component;
        }
        else
        {
            // doesnt exist
            self.sparse[id] = self.dense.len();
            self.dense_entities.push(id);
            self.dense.push(component);
        }

        Ok(())
    }

    pub fn remove(&mut self, id: usize) -> Result<(), ()>
    {
        let sparse_idx = self.sparse[id];

        if sparse_idx == NPOS
        {
            return Err(());
        }

        let last = self.dense.len() - 1;

        if sparse_idx != last
        {
            self.dense.swap_remove(sparse_idx);
            self.dense_entities.swap_remove(sparse_idx);

            let dense_id = self.dense_entities[last];
            self.sparse[dense_id] = sparse_idx;
        }

        self.dense.pop();
        self.dense_entities.pop();

        self.sparse[id] = NPOS;

        Ok(())
    }

    pub fn get(&self, id: usize) -> Option<&T>
    {
        let sparse_idx = self.sparse[id];
        self.dense.get(sparse_idx)
    }

    pub fn get_mut(&mut self, id: usize) -> Option<&mut T>
    {
        let sparse_idx = self.sparse[id];
        self.dense.get_mut(sparse_idx)
    }

    pub fn len(&self) -> usize
    {
        self.dense.len()
    }
}

#[cfg(test)]
mod tests
{
    use crate::component::Component;
    use crate::sparse_set::SparseSet;

    #[derive(Component, Debug, PartialEq)]
    struct Position
    {
        x: f32,
        y: f32,
    }

    #[test]
    fn insert()
    {
        let id = 0;
        let mut sparse = SparseSet::<Position>::new();

        assert_eq!(sparse.insert(id, Position { x: 32.0, y: 32.0 }), Ok(()));
        assert_eq!(sparse.len(), 1);
        assert_eq!(sparse.get(id), Some(&Position { x: 32.0, y: 32.0 }));
    }

    #[test]
    fn remove()
    {
        let id = 0;
        let mut sparse = SparseSet::<Position>::new();
        sparse.insert(id, Position { x: 16.0, y: 16.0 }).unwrap();

        assert_eq!(sparse.get(id), Some(&Position { x: 16.0, y: 16.0 }));
        assert_eq!(sparse.len(), 1);
        assert_eq!(sparse.remove(id), Ok(()));
        assert_eq!(sparse.len(), 0);
        assert_eq!(sparse.remove(id), Err(()));
        assert_eq!(sparse.get(id), None);
    }
}
