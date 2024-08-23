use core::fmt::Debug;
use core::ptr;

use crate::arch::PAGE_SIZE;

use super::map::MemoryMap;
use super::MemoryRange;

/// Representation of a phyisical memory address
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub struct PhysicalAddr(usize);

impl Debug for PhysicalAddr {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "PhysicalAddr({:#x})", self.0)
    }
}

pub struct PhysicalMemoryManager {
    buffer: &'static mut [PhysicalAddr],
    total_pages: usize,
    first: usize,
    last: usize,
}

impl PhysicalMemoryManager {
    /// # Safety
    /// * Function assumes that all adresses passed in `memory_map` are always vaild and not used in any other way
    /// * Identity mapping is assumed during calling this function
    pub unsafe fn new(memory_map: &MemoryMap) -> PhysicalMemoryManager {
        if !memory_map.is_page_aligned() {
            panic!("Memory map is not page-aligned!");
        }
        let total_pages = memory_map.total_size() / PAGE_SIZE;

        // find a memory range that can hold a buffer
        let buffer_size_bytes = total_pages * size_of::<PhysicalAddr>();
        let buffer_size_pa = page_align(buffer_size_bytes);
        let (buffer_range_idx, buffer_range) = memory_map
            .available_areas()
            .iter()
            .enumerate()
            .find(|(_, r)| r.size() >= buffer_size_pa)
            .expect("Cannot find memory are that can hold pma buffer");

        // safety: this memory is not used otherwise, and is of size >= sizeof(usize) * total_pages
        // and we assume that they are always valid
        let buffer = unsafe {
            ptr::slice_from_raw_parts_mut(buffer_range.address() as *mut PhysicalAddr, total_pages)
                .as_mut()
                .expect("Invaild memory area")
        };

        let mut total_pages = 0;
        let mut dump_pages = |range: &MemoryRange| {
            let mut page = range.address();
            while page <= range.end_address() {
                buffer[total_pages] = PhysicalAddr(page);
                page += PAGE_SIZE;
                total_pages += 1;
            }
        };

        // dump pages that are left after creating buffer
        let rest_of_buffer_range = MemoryRange::from_start_and_end(
            buffer_range.address() + buffer_size_pa,
            buffer_range.end_address(),
        );
        dump_pages(&rest_of_buffer_range);

        // dump pages in other areas
        for (i, range) in memory_map.available_areas().iter().enumerate() {
            if i != buffer_range_idx {
                dump_pages(range)
            }
        }

        PhysicalMemoryManager {
            buffer,
            total_pages,
            first: 0,
            last: total_pages,
        }
    }

    pub fn request_page(&mut self) -> Option<PhysicalAddr> {
        if self.first == self.last {
            return None;
        }
        let area = self.buffer[self.first];
        self.first = (self.first + 1) % self.buffer.len();
        Some(area)
    }

    pub fn free_page(&mut self, page: PhysicalAddr) {
        self.buffer[self.last] = page;
        self.last = (self.last + 1) % self.buffer.len();
    }

    pub fn total_pages(&self) -> usize {
        self.total_pages
    }
}

fn page_align(val: usize) -> usize {
    if val % PAGE_SIZE == 0 {
        val
    } else {
        ((val / PAGE_SIZE) + 1) * PAGE_SIZE
    }
}
