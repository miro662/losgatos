use core::{alloc::GlobalAlloc, arch::asm};

use super::PAGE_SIZE;

struct Allocator;

struct AllocatorData {
    stack_pointer: usize,
}

const HEAP_START: *mut AllocatorData = 0x80300000 as *mut AllocatorData;

unsafe impl GlobalAlloc for Allocator {
    unsafe fn alloc(&self, layout: core::alloc::Layout) -> *mut u8 {
        let mut allocator_data = unsafe { HEAP_START.as_mut().unwrap() };
        let alignment_offset: usize =
            layout.align() - allocator_data.stack_pointer % layout.align();
        let addr = allocator_data.stack_pointer + alignment_offset;
        allocator_data.stack_pointer = addr + layout.size();
        addr as *mut u8
    }

    unsafe fn dealloc(&self, ptr: *mut u8, layout: core::alloc::Layout) {}
}

#[global_allocator]
static ALLOCATOR: Allocator = Allocator;

pub fn init_allocator() {
    let mut allocator_data = unsafe { HEAP_START.as_mut().unwrap() };
    allocator_data.stack_pointer = HEAP_START as usize + PAGE_SIZE;
}
