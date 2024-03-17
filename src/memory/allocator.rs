use core::{
    alloc::GlobalAlloc,
    sync::atomic::{AtomicUsize, Ordering},
};

use crate::debug::kdebug;

extern "C" {
    static _heap_start: u8;
}
static HEAP_POINTER: AtomicUsize = AtomicUsize::new(0x0);

struct Allocator;

unsafe impl GlobalAlloc for Allocator {
    unsafe fn alloc(&self, layout: core::alloc::Layout) -> *mut u8 {
        let heap_addr = unsafe { &_heap_start as *const u8 as usize };
        let _ = HEAP_POINTER.compare_exchange(0x0, heap_addr, Ordering::Relaxed, Ordering::Relaxed);
        let mut addr;
        'allocation_loop: loop {
            let heap_pointer = HEAP_POINTER.load(Ordering::Acquire);
            let previous_aligned = (heap_pointer / layout.align()) * layout.align();
            addr = if heap_pointer == previous_aligned {
                heap_pointer
            } else {
                previous_aligned + layout.align()
            };
            let new_heap_pointer = addr + layout.size();
            if HEAP_POINTER
                .compare_exchange(
                    heap_pointer,
                    new_heap_pointer,
                    Ordering::SeqCst,
                    Ordering::SeqCst,
                )
                .is_ok()
            {
                break 'allocation_loop;
            }
        }
        kdebug!("[memory allocator] allocated {:#x} for {:?}", addr, layout);
        addr as *mut u8
    }

    unsafe fn dealloc(&self, _ptr: *mut u8, _layout: core::alloc::Layout) {}
}

#[global_allocator]
static ALLOCATOR: Allocator = Allocator;
