#![no_std]
#![no_main]

extern crate alloc;
use core::arch::global_asm;

mod debug;
mod memory;
mod sbi;

use alloc::vec;
use core::panic::PanicInfo;
use debug::kdebug;

use crate::memory::init_allocator;

global_asm!(include_str!("entrypoint.S"));

/// Rust kernel entrypoint
///
/// Accepts a `hartid` as a parameter. This should be passed by OpenSBI in `a0` register
#[no_mangle]
pub extern "C" fn kernel_boot(hart_id: i32) -> ! {
    if hart_id != 0 {
        unsafe {
            sbi::hart_stop().unwrap_or_else(|_| loop {});
        }
    }

    kdebug!(include_str!("logo_fmt.txt"));

    init_allocator();
    let xd = vec![21, 37];
    let xd2 = vec![14, 88];
    kdebug!("{:?} {:?}", xd, xd2);

    panic!("no further actions")
}

#[panic_handler]
fn panic_handler(panic: &PanicInfo) -> ! {
    kdebug!("\n[ :( ] KERNEL PANIC\n{}\n", panic);
    loop {}
}
