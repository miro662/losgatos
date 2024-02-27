#![no_std]
#![no_main]

pub mod boot;
pub mod sbi;

use core::panic::PanicInfo;

#[panic_handler]
fn panic_handler(_panic: &PanicInfo) -> ! {
    let console_extension = sbi::DebugConsole::get_if_available().unwrap();
    console_extension.write_byte('P' as u8);
    loop {}
}
