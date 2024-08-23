#![no_std]
#![no_main]

mod arch;
mod data_structures;
mod debug;
mod memory;
mod sync;

use arch::wfi;
use core::panic::PanicInfo;
use debug::kdebug;
use memory::{
    map::MemoryMap,
    physical::{prepare_pma_buffer, PhysicalMemoryManager},
};

fn kernel_main(mut memory_map: MemoryMap) -> ! {
    let pma_buf = unsafe { prepare_pma_buffer(&mut memory_map) };
    let mut pma = PhysicalMemoryManager::new(&memory_map, pma_buf);
    kdebug!("{:?}", pma);
    let page_zero = pma.request_page().unwrap();
    kdebug!("{:?} - first page", page_zero);
    kdebug!("allocating {} pages", pma.total_pages() - 2);
    for _ in 0..(pma.total_pages() - 2) {
        let _ = pma.request_page();
    }
    kdebug!("{:?} - last page", pma.request_page());
    kdebug!(
        "{:?} - page that could not be allocated",
        pma.request_page()
    );
    pma.free_page(page_zero);
    kdebug!(
        "{:?} - page that could be allocated after freeing one",
        pma.request_page()
    );
    wfi()
}

#[panic_handler]
fn panic_handler(panic: &PanicInfo) -> ! {
    kdebug!("\nKERNEL PANIC");
    kdebug!("{}\n", panic);
    loop {}
}
