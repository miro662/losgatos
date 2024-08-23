use core::fmt::Debug;
use core::mem::transmute;
use core::ptr;
use core::sync::atomic::{AtomicUsize, Ordering};

use crate::arch::PAGE_SIZE;
use crate::debug::kdebug;
use crate::sync::AtomicMutex;

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
    total_pages: usize,
    first: AtomicMutex<usize>,
    last: AtomicMutex<usize>,
}

impl PhysicalMemoryManager {
    /// # Safety
    /// * Function assumes that all adresses passed in `memory_map` are vaild and not used in any other way
    /// * Identity mapping is assumed
    pub unsafe fn new(memory_map: &MemoryMap) -> PhysicalMemoryManager {
        let (mut first_page, mut last_page) = (0, 0);
        let mut total_pages = 0;
        for (i, area) in memory_map.available_areas().iter().enumerate() {
            let aa = area.address();
            let mut page = if aa % PAGE_SIZE == 0 {
                aa
            } else {
                ((aa / PAGE_SIZE) + 1) * PAGE_SIZE
            };
            if i == 0 {
                first_page = page;
            }

            while page <= area.end_address() {
                unsafe {
                    let page_ptr: *mut usize = transmute(page);
                    *page_ptr = last_page;
                }
                last_page = page;
                page += PAGE_SIZE;
                total_pages += 1;
            }
        }

        PhysicalMemoryManager {
            total_pages,
            first: AtomicMutex::new(last_page),
            last: AtomicMutex::new(first_page),
        }
    }

    pub fn request_page(&self) -> PhysicalAddr {
        let mut first = self.first.lock();
        let page = *first;
        *first = unsafe {
            let page_ptr: *const usize = transmute(page);
            *page_ptr
        };
        PhysicalAddr(*first)
    }

    pub fn free_page(&self, page: PhysicalAddr) {
        let mut last = self.last.lock();
        unsafe {
            let last_ptr: *mut usize = transmute(*last);
            *last_ptr = page.0;
        }
        *last = page.0;
    }

    pub fn total_pages(&self) -> usize {
        self.total_pages
    }
}
