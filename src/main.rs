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
    if hart_id != 0 {
        unsafe {
            sbi::hart_stop().unwrap_or_else(|_| loop {});
        }
    }

    kdebug!(include_str!("logo_fmt.txt"));

    let device_tree = DeviceTree::from_pointer(devicetree_ptr);
    device_tree.print();

    panic!("no further actions")
}

#[panic_handler]
fn panic_handler(panic: &PanicInfo) -> ! {
    kdebug!("\n[ :( ] KERNEL PANIC\n{}\n", panic);
    loop {}
}
