#![no_std]
#![no_main]

extern crate alloc;
use core::arch::global_asm;

mod debug;
mod devicetree;
mod memory;
mod sbi;
mod sync;

use core::ffi::c_void;
use core::panic::PanicInfo;
use debug::kdebug;

use crate::devicetree::DeviceTree;

global_asm!(include_str!("entrypoint.S"));

/// Rust kernel entrypoint
///
/// Accepts a `hartid` as a parameter. This should be passed by OpenSBI in `a0` register
#[no_mangle]
pub extern "C" fn kernel_boot(hart_id: i32, devicetree_ptr: *const c_void) -> ! {
    kdebug!(include_str!("logo_fmt.txt"));
    kdebug!("Booting from hart {}", hart_id);
    let device_tree = DeviceTree::load_default(devicetree_ptr);
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
