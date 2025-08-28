use thiserror::Error;

#[derive(Error, Debug)]
pub enum EntityError {
    #[error("index {got} exceeds max {max} (INDEX_BITS={bits})")]
    IndexTooLarge { got: u32, max: u32, bits: u32 },

    #[error("version {got} exceeds max {max} (INDEX_BITS={bits})")]
    VersionTooLarge { got: u32, max: u32, bits: u32 },

    #[error("entity {id} is not alive")]
    NotAlive { id: u32 },
}
