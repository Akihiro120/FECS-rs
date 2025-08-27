use crate::entity_error::EntityError;

const INDEX_BITS: u32 = 20;
const VERSION_BITS: u32 = 12;
const INDEX_MASK: u32 = (1u32 << INDEX_BITS) - 1;
const VERSION_MASK: u32 = ((1u32 << VERSION_BITS) - 1) << INDEX_BITS;

struct EntityBuilder
{
    index: u32,
    version: u32
}

impl EntityBuilder
{
    pub fn new() -> EntityBuilder
    {
        EntityBuilder 
        { 
            index: 0,
            version: 0
        }
    }

    pub fn set_index(&mut self, index: u32) -> &mut EntityBuilder
    {
        self.index = index;

        self
    }

    pub fn set_version(&mut self, version: u32) -> &mut EntityBuilder
    {
        self.version = version;

        self
    }

    pub fn build(&self) -> Result<Entity, EntityError>
    {
        if self.index > INDEX_MASK
        {
            return Err(EntityError::IndexTooLarge
                {
                    got: self.index,
                    max: INDEX_MASK,
                    bits: INDEX_BITS
                });
        }

        if self.version >= (1u32 << VERSION_BITS)
        {
            return Err(EntityError::VersionTooLarge
                {
                    got: self.version,
                    max: INDEX_MASK,
                    bits: INDEX_BITS
                });
        }

        let id = (self.version << INDEX_BITS) | (self.index & INDEX_MASK);
        return Ok(Entity 
        {
            id
        });   
    }
}

struct Entity 
{
    id: u32,
}

impl Entity {

    pub fn get_index(&self) -> u32
    {
        return self.id & INDEX_MASK;
    }

    pub fn get_version(&self) -> u32
    {
        return (self.id & VERSION_MASK) >> INDEX_BITS;
    }
}
