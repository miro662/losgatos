#![no_std]
#![no_main]

mod arch;
mod data_structures;
mod debug;
mod memory;
mod sync;

use arch::wfi;
use core::{mem, panic::PanicInfo};
use debug::kdebug;
use memory::{map::MemoryMap, physical::PhysicalMemoryManager};

fn kernel_main(memory_map: MemoryMap) -> ! {
    memory_map.describe();
    let pma = unsafe { PhysicalMemoryManager::new(memory_map) };
    for _ in 0..0x201 {
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
