use crate::{
    component::Component,
    fecs::Entity
};
use std::collections::HashMap;

pub struct SparseSet<T> {
    sparse: HashMap<Entity, u32>,
    dense: Vec<T>,
    entities: Vec<Entity>
}

impl<T> SparseSet<T> {
    pub fn new() -> SparseSet<T> {
        SparseSet {
            sparse: HashMap::new(),
            dense: Vec::new(),
            entities: Vec::new()
        }
    }

    pub fn insert(&mut self, id: &Entity, c: T) {
        if let None = self.sparse.get(&id) {
            self.dense.push(c);
            self.entities.push(id.clone());
            self.sparse.insert(id.clone(), (self.dense.len() - 1) as u32);
        }
    }

    pub fn remove(&mut self, id: &Entity) {
        if let Some(&dense_index) = self.sparse.get(&id) {
            let last_index = self.dense.len() - 1;

            // swap
            self.sparse.remove(id);

            if dense_index != last_index as u32 {
                self.dense.swap(dense_index as usize, last_index);
                self.entities.swap(dense_index as usize, last_index);

                // replace
                let swap_entity = self.entities[dense_index as usize];
                self.sparse.insert(swap_entity, dense_index);
            }

            self.dense.pop();
            self.entities.pop();
        }
    }

    pub fn get(&self, id: &Entity) -> Option<&T> {
        if let Some(s_id) = self.sparse.get(&id) {
            return self.dense.get(*s_id as usize);
        }
        return None;
    }

    pub fn get_mut(&mut self, id: &Entity) -> Option<&mut T> {
        if let Some(s_id) = self.sparse.get(&id) {
            return self.dense.get_mut(*s_id as usize);
        }
        return None;

    }
}

#[cfg(test)] 
mod test {
    use crate::sparse_set::SparseSet;

    #[test]
    fn test_insert_and_get() {
        let mut set: SparseSet<&str> = SparseSet::new();

        // Insert some components for different entities.
        set.insert(&1, "Component1");
        set.insert(&2, "Component2");
        set.insert(&3, "Component3");

        // Test that we can retrieve the components.
        assert_eq!(set.get(&1), Some(&"Component1"));
        assert_eq!(set.get(&2), Some(&"Component2"));
        assert_eq!(set.get(&3), Some(&"Component3"));
        // Requesting a non-existent entity returns None.
        assert_eq!(set.get(&4), None);
    }

    #[test]
    fn test_insert_duplicate() {
        let mut set: SparseSet<&str> = SparseSet::new();

        // Insert the same entity twice.
        set.insert(&1, "Component1");
        set.insert(&1, "ShouldNotReplace");

        // Duplicate insertion should not replace the original.
        assert_eq!(set.get(&1), Some(&"Component1"));
    }

    #[test]
    fn test_remove() {
        let mut set: SparseSet<&str> = SparseSet::new();

        // Insert multiple components.
        set.insert(&1, "Component1");
        set.insert(&2, "Component2");
        set.insert(&3, "Component3");

        // Remove an element in the middle.
        set.remove(&2);

        // Entity 2 should now be missing.
        assert_eq!(set.get(&2), None);

        // The others should still be accessible.
        assert_eq!(set.get(&1), Some(&"Component1"));
        assert_eq!(set.get(&3), Some(&"Component3"));
    }

    #[test]
    fn test_get_mut() {
        let mut set: SparseSet<i32> = SparseSet::new();

        // Insert a component and then modify it.
        set.insert(&1, 10);
        if let Some(comp) = set.get_mut(&1) {
            *comp = 42;
        }
        assert_eq!(set.get(&1), Some(&42));
    }

    #[test]
    fn test_remove_last_element() {
        let mut set: SparseSet<&str> = SparseSet::new();

        // Insert a single component.
        set.insert(&1, "OnlyComponent");

        // Remove that element.
        set.remove(&1);

        // Now it should be gone.
        assert_eq!(set.get(&1), None);
        // The dense vector and entities vector should be empty.
        assert!(set.dense.is_empty());
        assert!(set.entities.is_empty());
        // The sparse mapping may still contain old data,
        // but get() should rely only on valid indices.
    }
}
