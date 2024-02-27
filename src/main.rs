#![no_std]
#![no_main]

pub mod boot;

use core::panic::PanicInfo;

#[panic_handler]
fn panic_handler(_panic: &PanicInfo) -> ! {
    loop {}
}
