#[cfg(test)]
mod sparse_set_test
{
    use fecs::{containers::SparseSet, manager::EntityManager, SPARSE_SET_SIZE};

    /// Helper: build a fresh set and manager
    fn setup<T>() -> (SparseSet<T>, EntityManager)
    {
        let set = SparseSet::new();
        let em = EntityManager::new();
        (set, em)
    }

    #[test]
    fn test_insert_and_get()
    {
        let (mut set, mut em) = setup::<i32>();
        let e1 = em.create(); // assume this returns an Entity
        let e2 = em.create();

        // initially empty
        assert_eq!(set.size(), 0);
        assert!(!set.has(e1));

        // insert two distinct entities
        set.insert(&em, e1, 10);
        set.insert(&em, e2, 20);

        assert_eq!(set.size(), 2);
        assert!(set.has(e1) && set.has(e2));

        // get returns correct references
        assert_eq!(*set.get(e1, &em).unwrap(), 10);
        assert_eq!(*set.get(e2, &em).unwrap(), 20);
    }

    #[test]
    fn test_insert_overwrite()
    {
        let (mut set, mut em) = setup::<String>();
        let e = em.create();

        set.insert(&em, e, "first".into());
        assert_eq!(set.size(), 1);
        assert_eq!(set.get(e, &em).unwrap().as_str(), "first");

        // insert again on same entity should overwrite
        set.insert(&em, e, "second".into());
        assert_eq!(set.size(), 1);
        assert_eq!(set.get(e, &em).unwrap().as_str(), "second");
    }

    #[test]
    fn test_remove_updates_dense_and_sparse()
    {
        let (mut set, mut em) = setup::<u8>();
        let e1 = em.create();
        let e2 = em.create();
        let e3 = em.create();

        set.insert(&em, e1, 1);
        set.insert(&em, e2, 2);
        set.insert(&em, e3, 3);

        // remove middle element
        set.remove(e2, &em);
        assert_eq!(set.size(), 2);
        assert!(!set.has(e2));
        // the elements at dense indices 0 and 1 should be e1 and e3 (maybe swapped)
        let remaining: Vec<_> = (0..set.size() as u32).map(|i| set.entity_at(i)).collect();
        assert!(remaining.contains(&e1) && remaining.contains(&e3));

        // removing last element should just pop
        set.remove(e3, &em);
        assert_eq!(set.size(), 1);
        assert!(set.has(e1));
        assert!(!set.has(e3));
    }

    #[test]
    fn test_get_mut_and_modify()
    {
        let (mut set, mut em) = setup::<i32>();
        let e = em.create();

        set.insert(&em, e, 100);
        {
            let val = set.get_mut(e, &em).expect("should exist");
            *val += 23;
        }
        assert_eq!(*set.get(e, &em).unwrap(), 123);
    }

    #[test]
    fn test_size_and_entity_at()
    {
        let (mut set, mut em) = setup::<char>();
        let entities: Vec<_> = (0..5).map(|_| em.create()).collect();
        for (i, &e) in entities.iter().enumerate()
        {
            set.insert(&em, e, (b'A' + i as u8) as char);
        }
        assert_eq!(set.size(), 5);
        for i in 0..5
        {
            assert_eq!(set.entity_at(i as u32), entities[i]);
        }
    }

    #[test]
    fn test_clear_and_has()
    {
        let (mut set, mut em) = setup::<()>();
        let e = em.create();
        set.insert(&em, e, ());
        assert!(set.has(e));

        set.clear();
        assert_eq!(set.size(), 0);
        assert!(!set.has(e));
    }

    #[test]
    fn test_reserve_grows_sparse_pages()
    {
        let (mut set, _em) = setup::<i32>();
        // choose amount larger than one page
        let amount = SPARSE_SET_SIZE * 3 + 5;
        set.reserve(amount);
        // we reserved space in dense; but sparse must have (amount/SPARSE_SET_SIZE +1) pages
        let expected_pages = (amount + SPARSE_SET_SIZE - 1) / SPARSE_SET_SIZE;
        assert_eq!(set.get_sparse().len(), expected_pages);
    }
}
