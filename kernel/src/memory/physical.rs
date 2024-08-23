use core::fmt::Debug;
use core::sync::atomic::{AtomicUsize, Ordering};

use crate::arch::PAGE_SIZE;

use super::map::MemoryMap;

/// Representation of a phyisical memory address
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub struct PhysicalAddr(usize);

impl Debug for PhysicalAddr {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "PhysicalAddr({:#x})", self.0)
    }
}

pub struct PhysicalMemoryManager {
    memory_map: MemoryMap,
    region: usize,
    next_page: AtomicUsize,
}

impl PhysicalMemoryManager {
    /// # Safety
    /// * Function assumes that all adresses
    pub unsafe fn new(memory_map: MemoryMap) -> PhysicalMemoryManager {
        let first_addr = memory_map.available_areas().first().unwrap().address();
        PhysicalMemoryManager {
            memory_map,
            region: 0,
            next_page: AtomicUsize::new(first_addr),
        }
    }

    pub fn request_page(&self) -> PhysicalAddr {
        let find_region = |addr| {
            self.memory_map
                .available_areas()
                .iter()
                .enumerate()
                .find(|(_, r)| r.contains(addr))
                .map(|(i, _)| i)
        };

        let page = 'next_page: loop {
            let page = self.next_page.load(Ordering::Acquire);
            let next_page = page + PAGE_SIZE;

            let current_region = find_region(page).expect("Previous address has to be vaild");
            let next_page = if let Some(_) = find_region(next_page) {
                next_page
            } else {
                self.memory_map.available_areas()[current_region + 1].address()
            };

            let exchange_result = self.next_page.compare_exchange(
                page,
                next_page,
                Ordering::Release,
                Ordering::Relaxed,
            );
            match exchange_result {
                Ok(page) => break 'next_page page,
                Err(_) => {}
            }
        };
        PhysicalAddr(page)
    }

    pub fn free_page(&self, _page: PhysicalAddr) {}
}
