use std::{
    any::{
        TypeId, Any
    },
    collections::HashMap
};
use crate::{component::Component, query::Query, bitset::Bitset, sparse_set::SparseSet};

pub type Entity = u32;

pub struct FECS {
    max_components: u32,
    free_ids: Vec<Entity>,
    next_id: u32,
    storage: HashMap<TypeId, Box<dyn Any + 'static>>,
    c_signatures: HashMap<TypeId, usize>,
    e_signatures: HashMap<Entity, Bitset>,
}

impl FECS {
    pub fn new() -> FECS {
        FECS {
            max_components: 16,
            free_ids: Vec::new(),
            next_id: 0,
            storage: HashMap::new(),
            c_signatures: HashMap::new(),
            e_signatures: HashMap::new()
        }
    }

    pub fn add_entity(&mut self) -> Entity {
        if let Some(id) = self.free_ids.pop() {
            return id;
        }

        self.next_id += 1;
        return self.next_id - 1;
    }

    pub fn remove_entity(&mut self, id: Entity) {
        // does the id already exist as an free id???
        if !self.free_ids.contains(&id) {
            self.free_ids.push(id);
        } 
    }

    pub fn register_component<T>(&mut self) 
    where 
        T: Component
    {
        let type_id = TypeId::of::<T>();
        let length = self.c_signatures.len();
        self.c_signatures.entry(type_id)
            .or_insert_with(|| {
                if length as u32 + 1 > self.max_components {
                    panic!("Max Component Count Reached!!!");
                }
                length
            }); 
    }

    pub fn attach<T>(&mut self, id: &Entity, c: T) 
    where 
        T: Component
    {
        let type_id = std::any::TypeId::of::<T>();

        // update the entity bit
        let bit = self.c_signatures.get(&type_id).unwrap();
        self.e_signatures.entry(*id)
            .and_modify(|signature| {
                signature.set(*bit as usize);
            })
            .or_insert_with(|| {
                let mut bitset = Bitset::new(self.max_components as usize);
                bitset.set(*bit as usize);
                bitset
            });

        // add to the storage if it exists
        // if not then insert it
        self.storage.entry(type_id)
            .or_insert_with(|| {
                return Box::new(SparseSet::<T>::new());
            })
            .downcast_mut::<SparseSet<T>>()
            .expect("Failed to Downcast to Storage")
            .insert(id, c);
    }

    pub fn detach<T>(&mut self, id: &Entity) 
    where 
        T: Component
    {
        // remove from sparse sets, and remove its signature from entity
        let type_id = TypeId::of::<T>();
        // does entity have such component?
        self.storage.entry(type_id)
            .and_modify(|storage| {
                storage.downcast_mut::<SparseSet<T>>()
                    .expect("Downcast Failed")
                    .remove(&id)
            });

        let bit = self.c_signatures.get(&type_id).unwrap();
        self.e_signatures.get_mut(&id).unwrap().reset(*bit as usize);
    }

    pub fn get<T>(&mut self, id: &Entity) -> Option<&T> 
    where 
        T: Component
    {
        let type_id = TypeId::of::<T>();
        self.storage.get(&type_id)
            .and_then(|storage| {
                storage.downcast_ref::<SparseSet<T>>()
            })
            .unwrap()
            .get(&id)
    }

    pub fn get_mut<T>(&mut self, id: &Entity) -> Option<&mut T>
    where 
        T: Component
    {
        let type_id = TypeId::of::<T>();
        self.storage.get_mut(&type_id)
            .and_then(|storage| {
                storage.downcast_mut::<SparseSet<T>>()
            })
            .unwrap()
            .get_mut(&id)
    }

    pub fn query<Q>(&self) -> Vec<Entity> 
    where 
        Q: Query
    {
        let query_signature = Q::signature(&self);
        let mut entities: Vec<Entity> = Vec::new();
        for (e, sig) in self.get_e_signatures() {
            if *sig == query_signature {
                entities.push(e.clone()); 
            }
        }

        return entities;
    }

    // getters
    pub fn get_c_signatures(&self) -> &HashMap<TypeId, usize> {
        &self.c_signatures
    }

    pub fn get_e_signatures(&self) -> &HashMap<Entity, Bitset> {
        &self.e_signatures
    }

    pub fn get_max_components(&self) -> usize {
        self.max_components as usize
    }
}

#[cfg(test)]
mod test {
    use crate::fecs::*;

    #[test]
    fn test_add_entity() {
        let mut fecs = FECS::new();
        let e0 = fecs.add_entity();
        let e1 = fecs.add_entity();

        assert_eq!(e0, 0);
        assert_eq!(e1, 1);
    }

    #[test]
    fn test_remove_entity() {
        let mut fecs = FECS::new();
        let mut e0 = fecs.add_entity();
        assert_eq!(e0, 0);

        fecs.remove_entity(e0);
        e0 = fecs.add_entity();
        assert_eq!(e0, 0);

        let e1 = fecs.add_entity();
        assert_eq!(e1, 1);

        fecs.remove_entity(e0);
        e0 = fecs.add_entity();
        assert_eq!(e0, 0);
    }

    #[test]
    fn test_attach() {
        let mut fecs = FECS::new();
        u32::register(&mut fecs);

        // entity 0
        let e0 = fecs.add_entity();
        fecs.attach(&e0, 42);

        assert_eq!(fecs.get(&e0), Some(42).as_ref());

        // entity 1
        let e1 = fecs.add_entity();
        fecs.attach(&e1, 12);

        assert_ne!(fecs.get(&e1), Some(32).as_ref());
        assert_eq!(fecs.get(&e1), Some(12).as_ref());
    }

    #[test]
    fn test_get() {
        let mut fecs = FECS::new();
        u32::register(&mut fecs);

        let e0 = fecs.add_entity();
        fecs.attach(&e0, 534);
        assert_eq!(fecs.get::<u32>(&e0), Some(534).as_ref());

        *fecs.get_mut::<u32>(&e0).unwrap() = 12;
        assert_eq!(fecs.get::<u32>(&e0), Some(12).as_ref());
    }

    #[test]
    fn test_detach() {
        // test detaching
        let mut fecs = FECS::new();
        u32::register(&mut fecs);
        
        let e0 = fecs.add_entity();
        fecs.attach(&e0, 123);
        assert_eq!(fecs.get::<u32>(&e0), Some(123).as_ref());

        // remove component, expecting a None
        fecs.detach::<u32>(&e0);
        assert_ne!(fecs.get::<u32>(&e0), Some(123).as_ref());
        assert_eq!(fecs.get::<u32>(&e0), None);
    }

    #[test]
    fn test_signature() {
        let mut fecs = FECS::new();
        u32::register(&mut fecs);
        f32::register(&mut fecs);

        let e0 = fecs.add_entity();
        fecs.attach(&e0, 23);

        // one bit 
        let mut e0_signature = fecs.get_e_signatures().get(&e0).unwrap();
        let mut e0_result = Bitset::new(fecs.get_max_components());
        e0_result.set(0);
        assert_eq!(*e0_signature == e0_result, true);

        // multi
        fecs.attach(&e0, 23.0);
        e0_signature = fecs.get_e_signatures().get(&e0).unwrap();
        e0_result.set(1);
        assert_eq!(*e0_signature == e0_result, true);

        // remove
        fecs.detach::<u32>(&e0);
        e0_signature = fecs.get_e_signatures().get(&e0).unwrap();
        e0_result.reset(0);
        assert_eq!(*e0_signature == e0_result, true);
    }

    #[test]
    fn test_querying() {
        let mut fecs = FECS::new();
        u32::register(&mut fecs);
        f32::register(&mut fecs);

        let e0 = fecs.add_entity(); 
        let e1 = fecs.add_entity(); 
        let e2 = fecs.add_entity(); 

        fecs.attach(&e0, 0);
        fecs.attach(&e1, 32);
        fecs.attach(&e2, 64);

        fecs.query::<u32>()
            .into_iter()
            .enumerate()
            .rev()
            .for_each(|(i, e)| {
                let result = fecs.get::<u32>(&e).unwrap();
                let expected = i as u32 * 32;
                assert_eq!(*result, expected);
            });

    }
}

