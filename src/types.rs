pub struct GlobalComponent;

pub type Entity = u32;
pub type ComponentIndex = u32;

pub const INVALID_ENTITY: Entity = Entity::MAX;
pub const SPARSE_SET_SIZE: usize = 2048;

pub const INDEX_BITS: u32 = 20;
pub const VERSION_BITS: u32 = 12;

pub const INDEX_MASK: u32 = (1 << INDEX_BITS) - 1;
pub const VERSION_MASK: u32 = !INDEX_MASK;

pub const NPOS: u32 = u32::MAX;

#[inline]
pub fn build_entity_index(index: u32, version: u32) -> Entity
{
    (version << INDEX_BITS) | (index & INDEX_MASK)
}

#[inline]
pub fn get_entity_index(e: Entity) -> u32
{
    e & INDEX_MASK
}

#[inline]
pub fn get_entity_version(e: Entity) -> u32
{
    (e & VERSION_MASK) >> INDEX_BITS
}
