use crate::manager::component_manager::Component;
use crate::manager::ComponentManager;
use crate::manager::EntityManager;
use crate::Entity;

pub struct Registry
{
    component_manager: ComponentManager,
    entity_manager: EntityManager,
}

impl Registry
{
    pub fn new() -> Registry
    {
        Registry {
            component_manager: ComponentManager::new(),
            entity_manager: EntityManager::new(),
        }
    }

    pub fn attach<T: Component>(&mut self, entity: Entity, component: T)
    {
        if !self.entity_manager.is_alive(entity)
        {
            eprintln!("Entity is not alive.");
        }

        self.component_manager.attach::<T>(entity, component);
    }

    pub fn detach<T: Component>(&mut self, entity: Entity)
    {
        if !self.entity_manager.is_alive(entity)
        {
            eprintln!("Entity is not alive.");
        }

        self.component_manager.detach::<T>(entity);
    }

    pub fn get<T: Component>(&self, entity: Entity) -> Option<&T>
    {
        if !self.entity_manager.is_alive(entity)
        {
            eprintln!("Entity is not alive.");
        }

        self.component_manager.get::<T>(entity)
    }

    pub fn get_mut<T: Component>(&mut self, entity: Entity) -> Option<&mut T>
    {
        if !self.entity_manager.is_alive(entity)
        {
            eprintln!("Entity is not alive.");
        }

        self.component_manager.get_mut::<T>(entity)
    }

    pub fn register<T: Component>(&mut self)
    {
        self.component_manager.register::<T>();
    }

    pub fn contains<T: Component>(&self, entity: Entity) -> bool
    {
        if !self.entity_manager.is_alive(entity)
        {
            eprintln!("Entity is not alive.");
        }

        self.component_manager.contains::<T>(entity)
    }
}
