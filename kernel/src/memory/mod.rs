pub mod map;
pub mod physical;
pub mod static_area;

use core::fmt::Debug;

use crate::arch::PAGE_SIZE;

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

    pub fn aligned_subset(&self, alignment: usize) -> MemoryRange {
        let address = if self.address % alignment != 0 {
            ((self.address % alignment) + 1) * alignment
        } else {
            self.address
        };

        let size = if self.size % alignment != 0 {
            (self.size % alignment) * alignment
        } else {
            self.size
        };

        MemoryRange::from_address_and_size(address, size)
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

impl From<(usize, usize)> for MemoryRange {
    fn from((address, size): (usize, usize)) -> Self {
        Self::from_address_and_size(address, size)
    }
}
