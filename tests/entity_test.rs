#[cfg(test)]
mod entity_test
{
    use fecs::{manager::EntityManager, types::*};

    #[test]
    fn create_yields_alive_entity()
    {
        let mut em = EntityManager::new();
        let e = em.create();
        // newly created entity should be alive
        assert!(em.is_alive(e));
        // index and version match
        let idx = get_entity_index(e) as usize;
        let ver = get_entity_version(e);
        assert_eq!(em.get_versions()[idx], ver);
    }

    #[test]
    fn destroy_makes_entity_dead()
    {
        let mut em = EntityManager::new();
        let e = em.create();
        assert!(em.is_alive(e));

        em.destroy(e);
        assert!(!em.is_alive(e));
    }

    #[test]
    fn destroy_bumps_version_and_reuse_index()
    {
        let mut em = EntityManager::new();
        let e1 = em.create();
        let idx1 = get_entity_index(e1);
        let ver1 = get_entity_version(e1);

        em.destroy(e1);
        // version should have incremented
        let ver_after = em.get_versions()[idx1 as usize];
        assert_eq!(ver_after, ver1 + 1);
        assert!(!em.is_alive(e1));

        // next create should reuse same index but new version
        let e2 = em.create();
        let idx2 = get_entity_index(e2);
        let ver2 = get_entity_version(e2);
        assert_eq!(idx2, idx1);
        assert_eq!(ver2, ver_after);
        assert!(em.is_alive(e2));
    }

    #[test]
    fn many_create_destroy_cycles()
    {
        let mut em = EntityManager::new();
        let mut seen = std::collections::HashSet::new();

        // create 100 entities
        for _ in 0..100
        {
            seen.insert(em.create());
        }
        assert_eq!(seen.len(), 100);

        // destroy all of them
        for &e in &seen
        {
            em.destroy(e);
            assert!(!em.is_alive(e), "Entity {:?} should be dead", e);
        }

        // create 100 more, should reuse freed slots
        let mut seen2 = std::collections::HashSet::new();
        for _ in 0..100
        {
            seen2.insert(em.create());
        }
        assert_eq!(seen2.len(), 100);
        // none of the newly created entities should coincide with the old ones with the same version:
        for &e_old in &seen
        {
            assert!(!seen2.contains(&e_old), "Old entity {:?} reappeared", e_old);
        }
    }

    #[test]
    fn reserve_only_reserves_capacity()
    {
        let mut em = EntityManager::new();
        em.reserve(50);
        // just check that reserve didn't break anything:
        let e = em.create();
        assert!(em.is_alive(e));
    }
}
