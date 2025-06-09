#[cfg(test)]
mod sparse_set_tests
{
    use fecs::{
        sparse_set::SparseSet,
        types::{build_entity_index, Entity, SPARSE_SET_SIZE},
    };

    /// Helper to create an Entity with a given index and version 0
    fn make_entity(index: u32) -> Entity
    {
        build_entity_index(index, 0)
    }

    #[test]
    fn insert_and_get_basic()
    {
        let mut set = SparseSet::new();
        let e0 = make_entity(0);

        set.insert(e0, 10);
        assert!(set.has(e0), "entity should be present after insert");
        assert_eq!(set.get(e0), Some(&10));
        assert_eq!(set.size(), 1);
        assert_eq!(set.entity_at(0), e0);
    }

    #[test]
    fn insert_overwrites_existing()
    {
        let mut set = SparseSet::new();
        let e0 = make_entity(0);

        set.insert(e0, 10);
        set.insert(e0, 20);
        assert_eq!(set.size(), 1);
        assert_eq!(set.get(e0), Some(&20));
    }

    #[test]
    fn remove_swaps_and_pops()
    {
        let mut set = SparseSet::new();
        let e0 = make_entity(0);
        let e1 = make_entity(1);

        set.insert(e0, 10);
        set.insert(e1, 20);
        assert_eq!(set.size(), 2);

        set.remove(e0);
        assert!(!set.has(e0));
        assert!(set.has(e1));
        assert_eq!(set.size(), 1);
        assert_eq!(set.entity_at(0), e1);
    }

    #[test]
    fn remove_nonexistent_is_noop()
    {
        let mut set = SparseSet::<u32>::new();
        let e0 = make_entity(0);

        // should not panic or change anything
        set.remove(e0);
        assert!(!set.has(e0));
        assert_eq!(set.size(), 0);
    }

    #[test]
    fn get_mut_allows_modify()
    {
        let mut set = SparseSet::new();
        let e0 = make_entity(0);

        set.insert(e0, 42);
        if let Some(v) = set.get_mut(e0)
        {
            *v = 99;
        }
        else
        {
            panic!("get_mut returned None");
        }
        assert_eq!(set.get(e0), Some(&99));
    }

    #[test]
    fn clear_wipes_everything()
    {
        let mut set = SparseSet::new();
        let e0 = make_entity(0);

        set.insert(e0, 123);
        set.clear();
        assert_eq!(set.size(), 0);
        assert!(!set.has(e0));
    }

    #[test]
    fn reserve_grows_sparse_pages()
    {
        let mut set = SparseSet::<u32>::new();
        let reserve_amount = 1000;
        set.reserve(reserve_amount);

        let expected_pages = (reserve_amount + SPARSE_SET_SIZE - 1) / SPARSE_SET_SIZE;
        assert_eq!(
            set.get_sparse().len(),
            expected_pages,
            "sparse.len() should match number of pages needed"
        );
    }
}
