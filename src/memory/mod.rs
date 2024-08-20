pub mod map;
pub mod static_area;

use core::{fmt::Debug, ptr::addr_of};

#[derive(Clone, Copy)]
pub struct MemoryRange {
    address: usize,
    size: usize,
}

impl MemoryRange {
    pub fn from_address_and_size(address: usize, size: usize) -> MemoryRange {
        MemoryRange { address, size }
    }

    pub fn from_start_and_end(start: usize, end: usize) -> MemoryRange {
        debug_assert!(start <= end);
        MemoryRange {
            address: start,
            size: end - start + 1,
        }
    }

    pub fn address(&self) -> usize {
        self.address
    }

    pub fn end_address(&self) -> usize {
        self.address + self.size - 1
    }

    pub fn size(&self) -> usize {
        self.size
    }
}

impl Debug for MemoryRange {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(
            f,
            "{:#x} - {:#x} ({} bytes)",
            self.address,
            self.end_address(),
            self.size
        )
    }
}
