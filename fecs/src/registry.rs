use crate::entity_allocator::EntityAllocator;
use crate::sparse_set::SparseSet;
use crate::{component::Component, registry_error::RegistryError};
use std::{
    any::{Any, TypeId},
    collections::{hash_map::Entry, HashMap},
};

pub struct Registry
{
    allocator: EntityAllocator,
    storages: HashMap<TypeId, Box<dyn Any>>,
    queries: HashMap<TypeId, Box<dyn Any>>,
}

impl Registry
{
    pub fn new() -> Self
    {
        Self {
            allocator: EntityAllocator::new(),
            storages: HashMap::new(),
            queries: HashMap::new(),
        }
    }

    pub fn with_capacity(len: usize) -> Self
    {
        Self {
            allocator: EntityAllocator::with_capacity(len),
            storages: HashMap::new(),
            queries: HashMap::new(),
        }
    }

    pub fn create(&mut self) -> Result<usize, RegistryError>
    {
        self.allocator.create().map_err(|e| RegistryError::from(e))
    }

    pub fn destroy(&mut self, id: usize) -> Result<(), RegistryError>
    {
        self.allocator
            .destroy(id)
            .map_err(|e| RegistryError::from(e))
    }

    pub fn attach<T: Component>(&mut self, id: usize, component: T) -> Result<(), RegistryError>
    {
        let type_id = TypeId::of::<T>();

        let storage = match self.storages.entry(type_id)
        {
            Entry::Occupied(e) => e.into_mut(),
            Entry::Vacant(e) => e.insert(Box::new(SparseSet::<T>::new())),
        };

        storage
            .downcast_mut::<SparseSet<T>>()
            .expect("TypeId should be of type SparseSet<T>")
            .insert(id, component)?;

        Ok(())
    }

    pub fn detach<T: Component>(&mut self, id: usize) -> Result<(), RegistryError>
    {
        let type_id = TypeId::of::<T>();

        let storage = self
            .storages
            .get_mut(&type_id)
            .ok_or(RegistryError::NoSuchComponentSet)?;

        storage
            .downcast_mut::<SparseSet<T>>()
            .expect("TypeId should be of type SparseSet<T>")
            .remove(id)?;

        Ok(())
    }

    pub fn get<T: Component>(&self, id: usize) -> Option<&T>
    {
        let type_id = TypeId::of::<T>();

        self.storages
            .get(&type_id)?
            .downcast_ref::<SparseSet<T>>()
            .expect("TypeId should be of type SparseSet<T>")
            .get(id)
    }

    pub fn get_mut<T: Component>(&mut self, id: usize) -> Option<&mut T>
    {
        let type_id = TypeId::of::<T>();

        self.storages
            .get_mut(&type_id)?
            .downcast_mut::<SparseSet<T>>()
            .expect("TypeId should be of type SparseSet<T>")
            .get_mut(id)
    }

    pub fn len<T: Component>(&self) -> Result<usize, RegistryError>
    {
        let type_id = TypeId::of::<T>();

        let len = self
            .storages
            .get(&type_id)
            .ok_or(RegistryError::NoSuchComponentSet)?
            .downcast_ref::<SparseSet<T>>()
            .expect("TypeId should be of type SparseSet<T>")
            .len();

        Ok(len)
    }

    pub fn reserve<T: Component>(&mut self, len: usize)
    {
        let type_id = TypeId::of::<T>();

        let storage = match self.storages.entry(type_id)
        {
            Entry::Occupied(e) => e.into_mut(),
            Entry::Vacant(e) => e.insert(Box::new(SparseSet::<T>::new())),
        };

        storage
            .downcast_mut::<SparseSet<T>>()
            .expect("TypeId should be of type SparseSet<T>")
            .reserve(len);
    }

    pub fn has<T: Component>(&mut self, id: usize) -> bool
    {
        let type_id = TypeId::of::<T>();

        self.storages[&type_id]
            .downcast_ref::<SparseSet<T>>()
            .expect("TypeId should be of type SparseSet<T>")
            .has(id)
    }

    pub fn storage<T: Component>(&self) -> Option<&SparseSet<T>>
    {
        let type_id = TypeId::of::<T>();

        Some(
            self.storages[&type_id]
                .downcast_ref::<SparseSet<T>>()
                .expect("TypeId should be of type SparseSet<T>"),
        )
    }
}

#[cfg(test)]
mod tests
{

    use crate::component::Component;
    use crate::registry::Registry;
    use crate::registry_error::RegistryError;
    use crate::sparse_error::SparseSetError;

    #[derive(Component, Debug, PartialEq)]
    struct Position
    {
        x: f32,
        y: f32,
    }

    #[test]
    pub fn create()
    {
        let mut registry = Registry::new();

        assert_eq!(registry.create().ok(), Some(0));
        assert_eq!(registry.create().ok(), Some(1));
    }

    #[test]
    pub fn destroy()
    {
        let mut registry = Registry::new();

        let e0 = registry.create().unwrap();
        let e1 = registry.create().unwrap();
        let _ = registry.create().unwrap();

        assert_eq!(registry.destroy(e0).is_ok(), true);
        assert_eq!(registry.destroy(e1).is_ok(), true);

        assert_eq!(registry.create().ok(), Some(1));
        assert_eq!(registry.create().ok(), Some(0));
    }

    #[test]
    pub fn attach()
    {
        let mut registry = Registry::new();

        let e0 = registry.create().unwrap();

        assert_eq!(
            registry.attach(e0, Position { x: 32.0, y: 32.0 }).ok(),
            Some(())
        );

        assert_eq!(
            registry.get::<Position>(e0),
            Some(&Position { x: 32.0, y: 32.0 })
        );

        assert_eq!(
            registry.attach(e0, Position { x: 128.0, y: 128.0 }).ok(),
            Some(())
        );

        assert_eq!(
            registry.get::<Position>(e0),
            Some(&Position { x: 128.0, y: 128.0 })
        );

        let e1 = registry.create().unwrap();

        assert_eq!(
            registry.attach(e1, Position { x: 11.0, y: 12.0 }).ok(),
            Some(())
        );

        assert_eq!(
            registry.get::<Position>(e1),
            Some(&Position { x: 11.0, y: 12.0 })
        );
    }

    #[test]
    pub fn detach()
    {
        let mut registry = Registry::new();

        let e0 = registry.create().unwrap();

        assert_eq!(
            registry.detach::<Position>(e0).err(),
            Some(RegistryError::NoSuchComponentSet)
        );
        registry.attach(e0, Position { x: 32.0, y: 32.0 }).unwrap();
        assert_eq!(
            registry.get::<Position>(e0),
            Some(&Position { x: 32.0, y: 32.0 })
        );
        assert_eq!(registry.detach::<Position>(e0).ok(), Some(()));
        assert_eq!(
            registry.detach::<Position>(e0).err(),
            Some(RegistryError::SparseSet(SparseSetError::InvalidPosition))
        );
    }

    #[test]
    pub fn query_2()
    {
        let mut registry = Registry::new();
    }

    #[test]
    pub fn query_4()
    {
        let mut registry = Registry::new();
    }
}
