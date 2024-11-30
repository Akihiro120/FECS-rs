use super::{FECS, Component};
use std::rc::Rc;
use std::cell::RefCell;
use std::cmp::{Eq, PartialEq};

pub struct Entity {
    id: u32,
    fecs: Rc<FECS>
}

impl Entity {
    pub fn new(id: u32, fecs: Rc<FECS>) -> Entity {
        Self {
            id,
            fecs,
        } 
    }

    /*
    ! this function will use a Rc to the ECS, and will attempt to add a component to this entity.
    ! This function will panic! on error
    */
    pub fn add_component<T: Component>(&self, component: T) -> &Self {
        self.fecs.add_component(&self, component).unwrap();
        self
    }

    /*
    ! this function will use a Rc to the ECS, and will attempt to remove a component to this entity.
    ! this function will panic! on error
    */
    pub fn remove_component<T: Component>(&self) -> &Self {
        self.fecs.remove_component::<T>(&self).unwrap();
        self
    }

    /*
    ! this function will use a Rc to the ECS, and will attempt to get a component to this entity.
    ! this function will panic! on error
    */
    pub fn get_component<T: Component>(&self) -> Option<Rc<RefCell<T>>> {
        self.fecs.get_component::<T>(&self)
    }

}

impl From<Entity> for u32 {
    fn from(e: Entity) -> Self {
        return e.id;
    }
}

impl From<&Entity> for u32 {
    fn from(e: &Entity) -> u32 {
        return e.id;
    }
}

// debug
use std::fmt;

impl fmt::Debug for Entity {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("Entity ")
            .field("id", &self.id)
            .finish()
    }
}

impl fmt::Display for Entity {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("Entity ")
            .field("id", &self.id)
            .finish()
    }
}

// partial eq
impl PartialEq for Entity {
    fn eq(&self, other: &Self) -> bool {
        self.id == other.id
    }
}

