//! Custom memory allocators.

use crate::c_api::*;
use std::mem;

/// Simple region based allocator. Allocates continuous chunk of memory with a specific size.
/// Allocator maintain a pointer within that memory, whenever allocate an object,
/// update the pointer by the object's size.
pub struct RegionAllocator {
    region: RegionMemoryBuffer,
}

unsafe impl Send for RegionAllocator {}

impl RegionAllocator {
    /// Create a new allocator with a specific size.
    pub fn new(size: usize) -> Self {
        Self {
            region: unsafe { create_region_memory_buffer(size as u64) },
        }
    }

    /// Allocate a new chunk of memory with a specific size.
    /// returns the base address of the allocated chunk of memory.
    ///
    /// # Errors
    ///
    /// If the memory is run out, then this call will return an error.
    pub unsafe fn alloc(&mut self, size: usize) -> Result<*mut u8, &'static str> {
        let data =
            region_memory_buffer_alloc(&mut self.region as *mut RegionMemoryBuffer, size as u64);

        if data.is_null() {
            Err("Out of memory")
        }
        else {
            Ok(data)
        }
    }

    /// Free all memory.
    pub unsafe fn clear(&mut self) -> Result<(), &'static str> {
        region_memory_buffer_free(&mut self.region as *mut RegionMemoryBuffer);
        Ok(())
    }

    /// Allocate a new region of memory with size = `size` of T and emplace a `value`
    /// to the allocated memory.
    ///
    /// Returns a pointer to the struct located in the memory of the allocator.
    pub unsafe fn emplace_struct<T>(&mut self, value: T) -> Result<*mut T, &'static str> {
        let value_ptr = &value as *const T;
        let data = region_memory_buffer_emplace(
            &mut self.region as *mut RegionMemoryBuffer,
            mem::size_of::<T>() as u64,
            value_ptr as *const u8,
        );

        if data.is_null() {
            Err("Out of memory")
        }
        else {
            Ok(data as *mut T)
        }
    }
}

#[cfg(test)]
mod tests {
    use assert_approx_eq::assert_approx_eq;

    use crate::allocator::*;
    use crate::data::*;

    #[test]
    fn test_region_allocator_emplace_struct() {
        unsafe {
            let mut allocator = RegionAllocator::new(1024);
            let vec = Vec2f::new(10., 20.);
            let vec = allocator.emplace_struct(vec).unwrap().as_ref().unwrap();

            assert_eq!(mem::size_of::<Vec2f>(), allocator.region.offset);
            assert_approx_eq!(10., vec.x);
            assert_approx_eq!(20., vec.y);
        }
    }
}
