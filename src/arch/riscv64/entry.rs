use core::arch::global_asm;

use crate::{
    debug::kdebug,
    devicetree::flattened::{FdtHeader, FlattenedDeviceTree},
    kernel_main,
};

global_asm!(include_str!("entrypoint.S"));

#[no_mangle]
pub extern "C" fn kernel_boot(_hart_id: i32, devicetree_ptr: *const FdtHeader) -> ! {
    // SAFETY: this is expected to point to a vaild DeviceTree
    let flattened_devicetree =
        unsafe { FlattenedDeviceTree::from_ptr(devicetree_ptr) }.expect("Invaild DTB");
    let dt_root = flattened_devicetree.root();

    let memory_nodes = dt_root
        .children()
        .filter(|n| n.name().starts_with("memory@"));
    for mnode in memory_nodes {
        kdebug!("{}", mnode.name());
    }
    kernel_main()
}
