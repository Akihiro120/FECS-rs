use crate::{containers::SparseSet, Entity};
use std::{any::TypeId, collections::HashMap};

pub trait Component: 'static {}

pub struct ComponentManager
{
    storage: HashMap<TypeId, Box<dyn std::any::Any>>,
}

impl ComponentManager
{
    pub fn new() -> ComponentManager
    {
        ComponentManager {
            storage: HashMap::new(),
        }
    }

    pub fn register<T: Component>(&mut self)
    {
        let component_type = TypeId::of::<T>();
        self.storage
            .entry(component_type)
            .or_insert_with(|| Box::new(SparseSet::<T>::new()));
    }

    pub fn attach<T: Component>(&mut self, entity: Entity, component: T)
    {
        self.register::<T>();
        let id = TypeId::of::<T>();
        let storage = self.storage.get_mut(&id).unwrap();
        let set = storage.downcast_mut::<SparseSet<T>>().unwrap();
        set.insert(entity, component);
    }

    pub fn detach<T: Component>(&mut self, entity: Entity)
    {
        self.register::<T>();
        let id = TypeId::of::<T>();
        let storage = self.storage.get_mut(&id).unwrap();
        let set = storage.downcast_mut::<SparseSet<T>>().unwrap();
        set.remove(entity)
    }

    pub fn get<T: Component>(&self, entity: Entity) -> Option<&T>
    {
        let id = TypeId::of::<T>();
        self.storage.get(&id).and_then(|storage| {
            let set = storage.downcast_ref::<SparseSet<T>>().unwrap();
            set.get(entity)
        })
    }

    pub fn get_mut<T: Component>(&mut self, entity: Entity) -> Option<&mut T>
    {
        let id = TypeId::of::<T>();
        self.storage.get_mut(&id).and_then(|storage| {
            let set = storage.downcast_mut::<SparseSet<T>>().unwrap();
            set.get_mut(entity)
        })
    }

    pub fn contains<T: Component>(&self, entity: Entity) -> bool
    {
        let id = TypeId::of::<T>();
        let storage = self.storage.get(&id).unwrap();
        let set = storage.downcast_ref::<SparseSet<T>>().unwrap();

        set.has(entity)
    }
}

// Supply default implementations of Component
