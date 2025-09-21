use crate::component::Component;

struct SparseSet<T>
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
        Ok(())
    }

    pub fn remove(&mut self, id: usize) -> Result<(), ()>
    {
        Ok(())
    }

    pub fn size(&self) -> usize
    {
        self.dense.len()
    }

    pub fn get(&mut self, id: usize) -> Option<&T>
    {
    }

    pub fn get_mut(&mut self, id: usize) -> Option<&mut T>
    {
    }
}
