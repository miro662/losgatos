use core::arch::global_asm;

use crate::{
    debug::kdebug,
    devicetree::flattened::{DeviceTreeHeader, FlattenedDeviceTree},
    kernel_main,
};

global_asm!(include_str!("entrypoint.S"));

#[no_mangle]
pub extern "C" fn kernel_boot(hart_id: i32, devicetree_ptr: *const DeviceTreeHeader) -> ! {
    // SAFETY: this is expected to point to a vaild DeviceTree
    let flattened_devicetree =
        unsafe { FlattenedDeviceTree::from_ptr(devicetree_ptr) }.expect("Invaild DTB");
    let dt_root = flattened_devicetree.root();
    for (name, value) in dt_root.properties() {
        kdebug!("{}: {:?}", name, value);
    }
    kernel_main()
}
