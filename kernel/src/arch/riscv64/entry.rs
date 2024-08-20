use core::arch::global_asm;

use devicetree::{FdtHeader, FlattenedDeviceTree};

use crate::{kernel_main, memory::map::MemoryMap};

global_asm!(include_str!("entrypoint.S"));

#[no_mangle]
pub extern "C" fn kernel_boot(_hart_id: i32, devicetree_ptr: *const FdtHeader) -> ! {
    // SAFETY: this is expected to point to a vaild DeviceTree
    let flattened_devicetree =
        unsafe { FlattenedDeviceTree::from_ptr(devicetree_ptr) }.expect("Invaild device tree");
    let dt_root = flattened_devicetree
        .root()
        .expect("Cannot retrieve device tree root");
    let memory_map = MemoryMap::from_device_tree(&dt_root).expect("Cannot read memory map");

    kernel_main(memory_map)
}
