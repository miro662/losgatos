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
    pub fn new(
        memory_map: &MemoryMap,
        buffer: &'static mut [PhysicalAddr],
    ) -> PhysicalMemoryManager {
        let mut total_pages = 0;
        for range in memory_map.available_areas() {
            let aligned_range = range.aligned_subset(PAGE_SIZE);
            let mut page = aligned_range.address();
            while page <= aligned_range.end_address() {
                buffer[total_pages] = PhysicalAddr(page);
                page += PAGE_SIZE;
                total_pages += 1;
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
        self.buffer[self.last % self.buffer.len()] = page;
        self.last = (self.last + 1) % self.buffer.len();
    }

    pub fn total_pages(&self) -> usize {
        self.total_pages
    }
}

impl Debug for PhysicalMemoryManager {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_struct("PhysicalMemoryManager")
            .field("total_pages", &self.total_pages)
            .field("first", &self.first)
            .field("last", &self.last)
            .finish()
    }
}

pub unsafe fn prepare_pma_buffer(memory_map: &mut MemoryMap) -> &'static mut [PhysicalAddr] {
    memory_map.page_align();

    let total_pages = memory_map.total_size() / PAGE_SIZE;

    let size = page_align(total_pages * size_of::<PhysicalAddr>());
    let range = memory_map
        .available_areas_mut()
        .iter_mut()
        .find(|r| r.size() >= size)
        .expect("Cannot find memory are that can hold pma buffer");
    let address = range.address();
    *range = MemoryRange::from_start_and_end(address + size, range.end_address());

    // safety: this memory is not used otherwise, and is of size >= sizeof(usize) * total_pages
    // and we assume that they are always valid
    unsafe {
        ptr::slice_from_raw_parts_mut(address as *mut PhysicalAddr, total_pages)
            .as_mut()
            .expect("Invaild memory area")
    }
}

fn page_align(val: usize) -> usize {
    if val % PAGE_SIZE == 0 {
        val
    } else {
        ((val / PAGE_SIZE) + 1) * PAGE_SIZE
    }
}
