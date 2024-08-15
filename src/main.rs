#![no_std]
#![no_main]

mod arch;
mod debug;
mod sync;

use core::panic::PanicInfo;
use debug::kdebug;

fn kernel_main() -> ! {
    kdebug!("Initialization successful - entering endless loop");
    loop {}
}

#[panic_handler]
fn panic_handler(panic: &PanicInfo) -> ! {
    kdebug!("");
    kdebug!("===============================================================================");
    kdebug!("[ :( ] KERNEL PANIC");
    kdebug!("-------------------------------------------------------------------------------");
    kdebug!("{}\n", panic);
    loop {}
}
