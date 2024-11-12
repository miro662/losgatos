use core::arch::global_asm;

use devicetree::FdtHeader;

use crate::Supervisor;

global_asm!(include_str!("entry.S"));

#[no_mangle]
pub extern "C" fn entrypoint_rs(_hart_id: i32, devicetree_ptr: *const FdtHeader) -> ! {
    let supervisor = Supervisor::new();
    unsafe { supervisor.set_global() };
    supervisor.launch(devicetree_ptr);
}
