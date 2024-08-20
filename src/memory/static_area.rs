use core::ptr::addr_of;

use super::MemoryRange;

extern "C" {
    static mut _start: u8;
    static mut _heap_start: u8;
}

pub fn kernel_static_memory_area() -> MemoryRange {
    unsafe {
        let start_addr = addr_of!(_start) as usize;
        let end_addr = addr_of!(_heap_start) as usize;
        MemoryRange::from_start_and_end(start_addr, end_addr - 1)
    }
}
