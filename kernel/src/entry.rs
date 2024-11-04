use core::arch::global_asm;

use devicetree::FdtHeader;

use crate::{
    kernel_main, sbi,
    traps::{InterruptCode, Traps},
};

global_asm!(include_str!("entrypoint.S"));

#[no_mangle]
pub extern "C" fn kernel_boot(_hart_id: i32, _devicetree_ptr: *const FdtHeader) -> ! {
    let mut traps = unsafe { Traps::initialize() };
    traps.enable();
    traps.enable_interrupts(InterruptCode::Timer);
    unsafe {
        let _ = sbi::timer::set(400000);
    }
    kernel_main()
}
