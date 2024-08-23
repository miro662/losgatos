#![no_std]
#![no_main]

mod arch;
mod data_structures;
mod debug;
mod memory;
mod sync;

use arch::{wfi, PAGE_SIZE};
use core::{mem, panic::PanicInfo};
use debug::kdebug;
use memory::{map::MemoryMap, physical::PhysicalMemoryManager};

fn kernel_main(memory_map: MemoryMap) -> ! {
    memory_map.describe();
    let pma = unsafe { PhysicalMemoryManager::new(&memory_map) };
    kdebug!(
        "Initialized physical memory manager - {} pages ({} B) available",
        pma.total_pages(),
        pma.total_pages() * PAGE_SIZE
    );
    for _ in 0..0x10 {
        let ppage = pma.request_page();
        kdebug!("{:?}", ppage);
    }
    wfi()
}

#[panic_handler]
fn panic_handler(panic: &PanicInfo) -> ! {
    kdebug!("\nKERNEL PANIC");
    kdebug!("{}\n", panic);
    loop {}
}
