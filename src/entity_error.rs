use thiserror::Error;

#[derive(Error, Debug)]
pub enum EntityError {
    #[error("version {got} exceeds max {max}")]
    VersionTooLarge { got: u32, max: u32 },

    #[error("entity {id} is not alive")]
    NotAlive { id: u32 },

    #[error("index overflow (got {got} exceeds max {max})")]
    IndexOverflow { got: u32, max: u32 },

    #[error("corrupted index {idx} (no version entry)")]
    CorruptedIndex { idx: u32 },
}
