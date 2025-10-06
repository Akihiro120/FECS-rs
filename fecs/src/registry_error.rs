use crate::{allocator_error::AllocatorError, sparse_error::SparseSetError};

#[derive(Debug, PartialEq)]
pub enum RegistryError
{
    NoSuchEntity,
    NoSuchComponentSet,
    CapacityExceeded,
    SparseSet(SparseSetError),
    Allocator(AllocatorError),
}

impl From<SparseSetError> for RegistryError
{
    fn from(value: SparseSetError) -> Self
    {
        RegistryError::SparseSet(value)
    }
}

impl From<AllocatorError> for RegistryError
{
    fn from(value: AllocatorError) -> Self
    {
        RegistryError::Allocator(value)
    }
}
