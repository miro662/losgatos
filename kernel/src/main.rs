#![no_std]
#![no_main]

mod arch;
mod data_structures;
mod debug;
mod memory;
mod sync;

use arch::{wfi, PAGE_SIZE};
use core::panic::PanicInfo;
use debug::kdebug;
use memory::{
    map::MemoryMap,
    physical::{prepare_pma_buffer, PhysicalMemoryManager},
};

fn kernel_main(mut memory_map: MemoryMap) -> ! {
    memory_map.describe();
    let pma_buf = unsafe { prepare_pma_buffer(&mut memory_map) };
    memory_map.describe();
    let mut pma = PhysicalMemoryManager::new(&memory_map, pma_buf);
    kdebug!(
        "Initialized physical memory manager - {} pages ({} B) available",
        pma.total_pages(),
        pma.total_pages() * PAGE_SIZE
    );
    let page_zero = pma.request_page().unwrap();
    for _ in 0..32570 {
        let _ = pma.request_page();
    }
    kdebug!("{:?}", pma.request_page());
    kdebug!("{:?}", pma.request_page());
    pma.free_page(page_zero);
    kdebug!("{:?}", pma.request_page());
    wfi()
}

#[panic_handler]
fn panic_handler(panic: &PanicInfo) -> ! {
    kdebug!("\nKERNEL PANIC");
    kdebug!("{}\n", panic);
    loop {}
}
