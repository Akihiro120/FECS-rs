use super::{SparseSet, Entity, Component, ECSErrorHandle};
use std::collections::HashMap;
use std::any::{Any, TypeId};
use std::rc::Rc;
use std::cell::RefCell;

pub struct FECS {
    free_ids: RefCell<Vec<u32>>,
    next_entity: RefCell<u32>,

    // archetypes
    signatures: RefCell<HashMap<u32, u64>>,
    components: RefCell<HashMap<TypeId, Box<dyn Any>>>,
    component_index: RefCell<HashMap<TypeId, u32>>
}

impl FECS {
    pub fn new() -> Rc<FECS> {
        Rc::new(Self {
            free_ids: RefCell::new(Vec::new()),
            next_entity: RefCell::new(0),
            signatures: RefCell::new(HashMap::new()),
            components: RefCell::new(HashMap::new()),
            component_index: RefCell::new(HashMap::new())
        })
    }

    /*
    ! this function will create a new entity
    ! it will first use up any free ids, if not then it would make a new id.
    ! the resultant id will give ownership to the return value
    */
    pub fn add_entity(self: &Rc<FECS>) -> Entity {
        // get the last free id
        if let Some(id) = self.free_ids.borrow_mut().pop() {
            // get a free id
            return Entity::new(id, self.clone()); 
        } else {
            // create new id
            *self.next_entity.borrow_mut() += 1;

            return Entity::new(*self.next_entity.borrow() - 1, self.clone());
        }
    }

    /*
    ! this function will remove the entity from the database, and store 
    ! its value in the free ids storage
    */
    pub fn remove_entity(self: &Rc<FECS>, id: Entity) {
        let index = u32::from(id);

        // free the signatures and components

        // remove the entity, consume the value, and add to free ids
        //self.signatures.borrow_mut().remove(&index).unwrap();
        self.free_ids.borrow_mut().push(index);
    }

    /*
    ! this function will add the component 
    */
    pub fn add_component<T: Component>(self: &Rc<FECS>, id: &Entity, component: T) -> Result<(), ECSErrorHandle> {
        let type_id = std::any::TypeId::of::<T>();

        // sparse set creation
        match self.components.borrow_mut()
            .entry(type_id)
            .or_insert_with(|| Box::new(SparseSet::<T>::new()))
            .downcast_mut::<SparseSet<T>>()
            .expect("Downcast to Component Set Failed")
            .add(id, component) {
            Err(_) => {
                eprintln!("FECS Error: Component Attachment Fail");
                return Err(ECSErrorHandle::ComponentAttachmentFail);
            },
            _ => {}
        }

        return Ok(());
    }

    /*
    ! this function will remove the component 
    */
    pub fn remove_component<T: Component>(self: &Rc<FECS>, id: &Entity) -> Result<(), ECSErrorHandle> {


        Err(ECSErrorHandle::ComponentRemovalFail)
    }

    /*
    ! this function will get the component and return a Option<Rc<RefCell<T>>>
    */
    pub fn get_component<T: Component>(self: &Rc<FECS>, id: &Entity) -> Option<Rc<RefCell<T>>> {
        let type_id = std::any::TypeId::of::<T>();

        // check if the component exists
        match self.components.borrow_mut()
            .get_mut(&type_id) {
            Some(result) => {
                return result
                    .downcast_mut::<SparseSet<T>>()
                    .unwrap()
                    .get(id)
            },
            None => {
                eprintln!("FECS Error: Component Entry Doesn't Exist");
                return None;
            }
        }
    }

    /*
    ! this function will query a system, a system uses templates to specify which components to query
    ! and will automatically search for the matching entity signatures to quickly query the components.
    */
}
