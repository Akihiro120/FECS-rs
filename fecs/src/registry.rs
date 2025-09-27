use crate::component::Component;
use crate::entity_allocator::EntityAllocator;
use crate::sparse_set::SparseSet;
use std::{
    any::{Any, TypeId},
    collections::HashMap,
};

struct Registry
{
    allocator: EntityAllocator,
    storages: HashMap<TypeId, Box<dyn Any>>,
}

impl Registry
{
    pub fn new() -> Self
    {
        Self {
            allocator: EntityAllocator::new(),
            storages: HashMap::new(),
        }
    }

    pub fn create(&mut self) -> Result<usize, ()>
    {
        self.allocator.create()
    }

    pub fn destroy(&mut self, id: usize) -> Result<(), ()>
    {
        self.allocator.destroy(id)
    }

    pub fn attach<T: Component>(&mut self, id: usize, component: T) -> Result<(), ()>
    {
    }

    pub fn detach<T: Component>(&mut self, id: usize, component: T) -> Result<(), ()>
    {
    }

    pub fn get<T: Component>(&mut self, id: usize) -> Option<&T>
    {
    }

    pub fn get_mut<T: Component>(&mut self, id: usize) -> Option<&mut T>
    {
    }

    pub fn len<T: Component>(&self) -> usize
    {
    }

    pub fn reserve<T: Component>(&mut self, len: usize)
    {
        self.allocator.reserve(len);
    }
}

#[cfg(test)]
mod tests
{}
