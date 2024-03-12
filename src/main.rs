#![no_std]
#![no_main]

use core::arch::global_asm;

pub mod debug;
pub mod sbi;

use core::panic::PanicInfo;
use debug::kdebug;

global_asm!(include_str!("entrypoint.S"));

/// Rust kernel entrypoint
///
/// Accepts a `hartid` as a parameter. This should be passed by OpenSBI in `a0` register
#[no_mangle]
pub extern "C" fn kernel_boot(hart_id: i32) -> ! {
    if hart_id != 0 {
        loop {}
    }

    kdebug!(include_str!("logo_fmt.txt"));
    panic!("no further instructions");
}

#[panic_handler]
fn panic_handler(panic: &PanicInfo) -> ! {
    kdebug!("\n[ :( ] KERNEL PANIC\n{}\n", panic);
    loop {}
}
