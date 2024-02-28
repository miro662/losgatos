#![no_std]
#![no_main]

pub mod boot;
pub mod debug;
pub mod sbi;

use core::panic::PanicInfo;
use debug::kdebug;

#[panic_handler]
fn panic_handler(panic: &PanicInfo) -> ! {
    kdebug!("\n[ :( ] KERNEL PANIC\n{}\n", panic);
    loop {}
}
