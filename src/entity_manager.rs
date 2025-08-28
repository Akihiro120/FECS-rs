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
