use core::{arch::global_asm, ffi::c_void};

use crate::kernel_main;

global_asm!(include_str!("entrypoint.S"));

#[no_mangle]
pub extern "C" fn kernel_boot(_hart_id: i32, _devicetree_ptr: *const c_void) -> ! {
    kernel_main()
}
