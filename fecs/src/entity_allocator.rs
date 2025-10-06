use crate::allocator_error::AllocatorError;

pub struct EntityAllocator
{
    next_id: usize,
    free_ids: Vec<usize>,
}

impl EntityAllocator
{
    pub fn new() -> Self
    {
        Self {
            next_id: 0,
            free_ids: Vec::new(),
        }
    }

    pub fn with_capacity(capacity: usize) -> Self
    {
        Self {
            next_id: 0,
            free_ids: Vec::with_capacity(capacity),
        }
    }

    pub fn create(&mut self) -> Result<usize, AllocatorError>
    {
        if let Some(free_id) = self.free_ids.pop()
        {
            Ok(free_id)
        }
        else
        {
            let idx = self.next_id;
            self.next_id += 1;
            Ok(idx)
        }
    }

    pub fn destroy(&mut self, id: usize) -> Result<(), AllocatorError>
    {
        self.free_ids.push(id);
        Ok(())
    }

    // used for memory debugging
    fn capacity(&self) -> usize
    {
        self.free_ids.capacity()
    }

    pub fn reserve(&mut self, amount: usize)
    {
        self.free_ids.reserve(amount);
    }
}

#[cfg(test)]
mod tests
{
    use crate::entity_allocator::EntityAllocator;

    #[test]
    fn create()
    {
        let mut allocator = EntityAllocator::new();

        assert_eq!(allocator.create(), Ok(0));
        assert_eq!(allocator.create(), Ok(1));
        assert_eq!(allocator.create(), Ok(2));
    }

    #[test]
    fn destroy()
    {
        let mut allocator = EntityAllocator::new();

        assert_eq!(allocator.create(), Ok(0));
        assert_eq!(allocator.create(), Ok(1));
        assert_eq!(allocator.create(), Ok(2));
        assert_eq!(allocator.destroy(2), Ok(()));
        assert_eq!(allocator.create(), Ok(2));
        assert_eq!(allocator.destroy(0), Ok(()));
        assert_eq!(allocator.create(), Ok(0));
    }

    #[test]
    fn ensure_capacity()
    {
        let allocator = EntityAllocator::with_capacity(123);
        assert_eq!(allocator.capacity(), 123);
    }

    #[test]
    fn reserve()
    {
        let mut allocator = EntityAllocator::new();
        allocator.reserve(123);
        assert_eq!(allocator.capacity(), 123);
    }
}
