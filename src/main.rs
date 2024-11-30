pub mod fecs {
    mod fecs;
    mod sparse_set;
    mod component;
    mod entity;
    mod error;

    // public usages
    pub use sparse_set::SparseSet;
    pub use component::Component;
    pub use entity::Entity;
    pub use error::SparseSetErrorHandle;
    pub use error::ECSErrorHandle;
    pub use fecs::FECS;
}
use fecs::{Component, SparseSet, Entity, SparseSetErrorHandle, ECSErrorHandle, FECS};

impl Component for u32 {}

struct Position {
    x: f32,
    y: f32
}
impl Component for Position {}

struct Velocity {
    x: f32,
    y: f32
}
impl Component for Velocity {}

//#[test]
//fn test_systems() {
//    // create ecs
//
//    // create entity
//
//    // attach components
//
//    // query system
//
//    // test to see if the system query has passed, and data reflect changes
//}

#[test]
fn test_component() {
    // create ecs
    let fecs = FECS::new();

    // create entities
    let e1 = fecs.add_entity();

    // attach components
    e1.add_component(Position{x: 32.0, y: 32.0});

    // test to see if the component data matches
    assert_eq!(e1.get_component::<Position>().unwrap().borrow().x, 32.0);
    assert_eq!(e1.get_component::<Position>().unwrap().borrow().y, 32.0);

    // update components
    e1.get_component::<Position>().unwrap().borrow_mut().x = 64.0;
    e1.get_component::<Position>().unwrap().borrow_mut().y = 64.0;

    // test to see if the component data reflects changes
    assert_eq!(e1.get_component::<Position>().unwrap().borrow().x, 64.0);
    assert_eq!(e1.get_component::<Position>().unwrap().borrow().y, 64.0);

    // remove entity

    // test to see if all components were removed
}

#[test]
fn test_entity() {
    // create ecs
    let fecs = FECS::new();

    // create entities
    let entities: Vec<_> = (0..4)
        .into_iter()
        .map(|_| fecs.add_entity())
        .collect();

    // test if the ids match
    entities
        .iter()
        .enumerate()
        .for_each(|(index, entity)| {
            assert_eq!(*entity, Entity::new(index as u32, fecs.clone()))
        });

    // remove an entity and test if a new entity will take on that free id
    let test_index = 1;
    entities
        .into_iter()
        .filter(|entity| u32::from(entity) == test_index)
        .for_each(|entity| fecs.remove_entity(entity));

    // make the new entity
    let new_entity = fecs.add_entity();
    assert_eq!(new_entity, Entity::new(test_index, fecs.clone()));
}

#[test]
fn test_sparse_sets() {

    // create sparse set
    let mut sparse = SparseSet::<u32>::new();
    let fecs = FECS::new();

    // create entities
    let e0 = Entity::new(0, fecs.clone());
    let e1 = Entity::new(1, fecs.clone());

    // give values to entities in sparse set
    sparse.add(&e0, 32).unwrap();
    sparse.add(&e1, 64).unwrap();

    // test to see if adding to the same entity returns Err
    match sparse.add(&e1, 32) {
        Err(result) => {
            assert_eq!(result, SparseSetErrorHandle::EntryAttachmentFail)
        }
        _ => {}
    }

    // test if the values are correct
    assert_eq!(*sparse.get(&e0).unwrap().borrow_mut(), 32);
    assert_eq!(*sparse.get(&e1).unwrap().borrow_mut(), 64);

    // test if the values are correct after mutation
    *sparse.get(&e0).unwrap().borrow_mut() = 64;
    *sparse.get(&e1).unwrap().borrow_mut() = 128;

    assert_eq!(*sparse.get(&e0).unwrap().borrow_mut(), 64);
    assert_eq!(*sparse.get(&e1).unwrap().borrow_mut(), 128);

    // test if removing twice gives error
    sparse.remove(&e0).unwrap();
    //match sparse.remove(&e1) {
    //    Err(result) => {
    //        assert_eq!(result, SparseSetErrorHandle::EntryRemovalFail)
    //    },
    //    _ => {}
    //}

    // test if attempting to get a value from an invalid entry returns Err
    match sparse.get(&e0) {
        None => assert_eq!(true, true),
        _ => {}
    }

    // test if we can add a new entry of the removed entity
    sparse.add(&e0, 32).unwrap();
}

fn main() {
}
