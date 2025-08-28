use crate::{
    entity::{Entity, EntityBuilder},
    entity_error::EntityError,
};

struct EntityManager {
    version: Vec<u32>,
    free: Vec<u32>,
}

impl EntityManager {
    pub fn new() -> EntityManager {
        Self {
            version: Vec::new(),
            free: Vec::new(),
        }
    }

    pub fn create(&mut self) -> Result<Entity, EntityError> {
        let idx: u32 = if let Some(i) = self.free.pop() {
            i
        } else {
            let next = self.version.len();
            let idx_u32 = u32::try_from(next).map_err(|_| EntityError::IndexOverflow {
                got: next as u32,
                max: u32::MAX,
            })?;

            self.version.push(0);

            idx_u32
        };

        let ver: u32 = *self
            .version
            .get(idx as usize)
            .ok_or(EntityError::CorruptedIndex { idx })?;

        EntityBuilder::new()
            .set_index(idx)
            .set_version(ver)
            .build()
            .map_err(EntityError::from)
    }

    pub fn destroy(&mut self, entity: Entity) {
        let idx = entity.get_index();
        if let Some(ver) = self.version.get_mut(idx as usize) {
            *ver += 1;
        }

        self.free.push(idx);
    }

    pub fn is_alive(&self, entity: Entity) -> bool {
        let idx = entity.get_index() as usize;
        let ver = entity.get_version();

        self.version
            .get(idx)
            .is_some_and(|stored_ver| *stored_ver == ver)
    }
}

#[cfg(test)]
mod entity_manager_tests {
    use super::*;
    use crate::entity::{Entity, EntityBuilder};

    #[test]
    fn create_returns_alive_entity() {
        let mut mgr = EntityManager::new();
        let e = mgr.create().expect("create should succeed");
        assert!(mgr.is_alive(e), "newly created entity should be alive");
    }

    #[test]
    fn destroy_makes_entity_dead() {
        let mut mgr = EntityManager::new();
        let e = mgr.create().unwrap();
        assert!(mgr.is_alive(e));

        mgr.destroy(e);
        assert!(!mgr.is_alive(e), "destroyed entity should not be alive");
    }

    #[test]
    fn index_is_reused_after_destroy() {
        let mut mgr = EntityManager::new();

        let e1 = mgr.create().unwrap();
        let idx1 = e1.get_index();
        let ver1 = e1.get_version();

        mgr.destroy(e1);
        assert!(!mgr.is_alive(e1));

        let e2 = mgr.create().unwrap();
        let idx2 = e2.get_index();
        let ver2 = e2.get_version();

        // free-list reuse: same index should be reused
        assert_eq!(idx1, idx2, "index should be reused from free list");

        // version must be bumped to prevent ABA
        assert_eq!(ver1.wrapping_add(1), ver2, "version should bump by 1");

        // old handle stays dead; new handle is alive
        assert!(mgr.is_alive(e2));
        assert!(!mgr.is_alive(e1));
    }

    #[test]
    fn multiple_creates_have_unique_handles_while_alive() {
        let mut mgr = EntityManager::new();
        let e1 = mgr.create().unwrap();
        let e2 = mgr.create().unwrap();

        // They can have different indices or versions, but as *live* handles they must differ.
        assert!(
            e1.get_index() != e2.get_index() || e1.get_version() != e2.get_version(),
            "two live entities should not be identical handles"
        );

        assert!(mgr.is_alive(e1));
        assert!(mgr.is_alive(e2));
    }

    #[test]
    fn double_destroy_is_safe_and_keeps_dead() {
        let mut mgr = EntityManager::new();
        let e = mgr.create().unwrap();
        mgr.destroy(e);
        // Destroying again should not panic; it just bumps version again and may push index again.
        mgr.destroy(e);
        assert!(!mgr.is_alive(e));
    }

    #[test]
    fn forged_handle_with_wrong_version_is_not_alive() {
        let mut mgr = EntityManager::new();
        let e = mgr.create().unwrap();
        let idx = e.get_index();
        let wrong_ver = e.get_version().wrapping_add(123);

        // Build a forged handle with the same index but wrong version.
        let forged = EntityBuilder::new()
            .set_index(idx)
            .set_version(wrong_ver)
            .build()
            .expect("builder should allow constructing arbitrary handle for test");

        assert!(!mgr.is_alive(forged), "forged handle must not be alive");
        assert!(mgr.is_alive(e), "original handle remains alive");
    }

    #[test]
    fn reused_index_after_two_cycles_has_incremented_version_twice() {
        let mut mgr = EntityManager::new();
        let e1 = mgr.create().unwrap();
        let idx = e1.get_index();
        let v1 = e1.get_version();

        mgr.destroy(e1);
        let e2 = mgr.create().unwrap();
        assert_eq!(e2.get_index(), idx);
        assert_eq!(e2.get_version(), v1.wrapping_add(1));
        mgr.destroy(e2);

        let e3 = mgr.create().unwrap();
        assert_eq!(e3.get_index(), idx);
        assert_eq!(e3.get_version(), v1.wrapping_add(2));
        assert!(mgr.is_alive(e3));
    }

    #[test]
    fn out_of_bounds_entity_is_not_alive() {
        let mgr = EntityManager::new();
        // No entities created; any index is out-of-bounds.
        let forged = EntityBuilder::new()
            .set_index(123456)
            .set_version(0)
            .build()
            .unwrap();

        assert!(!mgr.is_alive(forged));
    }

    #[test]
    fn many_creations_and_destroys_preserve_invariants() {
        let mut mgr = EntityManager::new();

        // Create a bunch
        let es: Vec<Entity> = (0..10_000).map(|_| mgr.create().unwrap()).collect();
        for &e in &es {
            assert!(mgr.is_alive(e));
        }

        // Destroy every other entity
        for (i, &e) in es.iter().enumerate() {
            if i % 2 == 0 {
                mgr.destroy(e);
                assert!(!mgr.is_alive(e));
            }
        }

        // Recreate the same count; indices from freed ones should be reused with bumped versions
        let recreated: Vec<Entity> = (0..(es.len() / 2)).map(|_| mgr.create().unwrap()).collect();
        for &e in &recreated {
            assert!(mgr.is_alive(e));
        }

        // All live entities are either the old odds or the newly recreated ones
        let mut live_count = 0usize;
        for &e in &es {
            if mgr.is_alive(e) {
                live_count += 1;
            }
        }
        live_count += recreated
            .iter()
            .filter(|&&e| !es.contains(&e) && mgr.is_alive(e))
            .count();

        // At least half should be live (the odds). This is a sanity check, not strict equality.
        assert!(live_count >= es.len() / 2);
    }
}
