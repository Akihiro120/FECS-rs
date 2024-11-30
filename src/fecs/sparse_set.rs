use std::collections::HashMap;
use super::{Entity, Component, SparseSetErrorHandle};
use std::rc::Rc;
use std::cell::RefCell;

pub struct SparseSet<T: Component> {
    dense: Vec<Rc<RefCell<T>>>,
    sparse: HashMap<u32, u32>,
    entities: Vec<u32>
}

impl<T: Component> SparseSet<T> {
    pub fn new() -> SparseSet<T> {
        Self {
            dense: Vec::new(),
            sparse: HashMap::new(),
            entities: Vec::new()
        }
    }

    /* 
    ! this function will return a Result<(), SparseSetErrorHandle>. 
    ! it will attempt to add an entry to the sparse set, if it fails it will return a 
    ! EntryAttachmentFail 
    */
    pub fn add(&mut self, id: &Entity, entry: T) -> Result<(), SparseSetErrorHandle> {
        let index = u32::from(id);

        // check if the id is in the registry
        if self.sparse.get(&index).is_none() {
            // add component to the dense map
            self.dense.push(Rc::new(RefCell::new(entry)));
            self.entities.push(index);

            // add the entity
            self.sparse
                .entry(index)
                .or_insert(self.dense.len() as u32 - 1);

            // return ok
            return Ok(());
        }

        return Err(SparseSetErrorHandle::EntryAttachmentFail);
    }

    /* this function will return a Result<(), SparseSetErrorHandle>. 
    ! it will attempt to remove an entry to the sparse set, if it fails it will return a 
    ! EntryRemovalFail 
    */
    pub fn remove(&mut self, id: &Entity) -> Result<(), SparseSetErrorHandle> {
        let index = u32::from(id);

        // check if id is in the registry
        if let Some(_sparse_index) = self.sparse.get(&index) {
            // get the current dense index
            let dense_index = self.sparse.get(&index).unwrap();

            // swap the last and current id
            self.dense.swap_remove(*dense_index as usize); 
            self.entities.swap_remove(*dense_index as usize);

            // replace
            self.sparse.insert(*self.entities.last().unwrap(), *dense_index);

            // remove
            self.sparse.remove(&index).unwrap();

            // return ok
            return Ok(());
        }

        return Err(SparseSetErrorHandle::EntryRemovalFail);
    }

    /* this function will return a Option<Rc<T>>. 
    ! it will attempt to get an entry to the sparse set, if it fails it will return a 
    ! None
    */
    pub fn get(&self, id: &Entity) -> Option<Rc<RefCell<T>>> {
        let index = u32::from(id);

        // check if the id is in the registry
        if let Some(sparse_index) = self.sparse.get(&index) {
           
            if let Some(dense_entry) = self.dense.get(*sparse_index as usize) {
                return Some(dense_entry.clone()); 
            }
        }

        return None;
    }
}
