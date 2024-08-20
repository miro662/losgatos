#![no_std]
#![no_main]

mod arch;
mod data_structures;
mod debug;
mod memory;
mod sync;

use core::panic::PanicInfo;
use debug::kdebug;
use memory::map::MemoryMap;

fn kernel_main(memory_map: MemoryMap) -> ! {
    memory_map.describe();
    kdebug!("Initialization successful - entering endless loop");
    #[allow(clippy::empty_loop)]
    loop {}
}

#[panic_handler]
fn panic_handler(panic: &PanicInfo) -> ! {
    kdebug!("\nKERNEL PANIC");
    kdebug!("{}\n", panic);
    loop {}
}
