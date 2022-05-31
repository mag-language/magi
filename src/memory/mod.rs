use std::ptr::NonNull;
use std::alloc::{alloc, dealloc, Layout};

pub type BlockPtr = NonNull<u8>;
pub type BlockSize = usize;

pub const BLOCK_SIZE_BITS: usize = 15;
pub const BLOCK_SIZE: usize      = 1 << BLOCK_SIZE_BITS;

pub struct Block {
    ptr:  BlockPtr,
    size: BlockSize,
}

impl Block {
    pub fn new(size: BlockSize) -> Result<Block, BlockError> {
        if !size.is_power_of_two() {
            return Err(BlockError::BadRequest);
        }

        Ok(Block {
            ptr: internal::allocate_block(size)?,
            size,
        })
    }

    pub fn as_pointer(&self) -> *const u8 {
        self.ptr.as_ptr()
    }
}

#[derive(Debug, PartialEq)]
pub enum BlockError {
    /// Usually means requested block size, and therefore alignment, wasn't a
    /// power of two
    BadRequest,
    /// Insufficient memory, couldn't allocate a block
    OutOfMemory,
}

pub mod internal {
    use super::*;

    pub fn allocate_block(size: BlockSize) -> Result<BlockPtr, BlockError> {
        unsafe {
            let layout  = Layout::from_size_align_unchecked(size, size);
            let pointer = alloc(layout);

            if !pointer.is_null() {
                Ok(NonNull::new_unchecked(pointer))
            } else {
                Err(BlockError::OutOfMemory)
            }

        }
    }

    pub fn deallocate_block(pointer: BlockPtr, size: BlockSize) {
        unsafe {
            let layout  = Layout::from_size_align_unchecked(size, size);
            dealloc(pointer.as_ptr(), layout)
        }
    }
}