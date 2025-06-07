use crate::types::{build_entity_index, get_entity_index, get_entity_version, Entity};

pub struct EntityManager
{
    versions: Vec<u32>,
    freelist: Vec<u32>,
}

impl EntityManager
{
    pub fn new() -> Self
    {
        return EntityManager {
            versions: Vec::new(),
            freelist: Vec::new(),
        };
    }

    pub fn reserve(&mut self, amount: usize)
    {
        self.versions.reserve(amount);
        self.freelist.reserve(amount);
    }

    pub fn create(&mut self) -> Entity
    {
        let idx: u32 = if let Some(free_index) = self.freelist.pop()
        {
            free_index
        }
        else
        {
            let next = self.versions.len() as u32;
            self.versions.push(0);

            next
        };

        let version = self
            .versions
            .get(idx as usize)
            .copied()
            .expect("Version doesn't exist for index");

        build_entity_index(idx, version)
    }

    pub fn destroy(&mut self, e: Entity)
    {
        let idx: u32 = get_entity_index(e);

        let version: &mut u32 = self
            .versions
            .get_mut(idx as usize)
            .expect("Version doesn't exist for index");
        *version += 1;

        self.freelist.push(idx);
    }

    pub fn is_alive(&self, e: Entity) -> bool
    {
        let idx: usize = get_entity_index(e) as usize;
        let ver: u32 = get_entity_version(e);

        return self.versions.get(idx).copied() == Some(ver);
    }
}
