use core::ptr::addr_of;

use devicetree::{FlattenedDeviceTree, NodeIterExt, NodeRef};

use crate::kdebug;

use super::types::{PhysicalAddr, PhysicalAddrRange};

pub struct MemoryMap;

extern "C" {
    static mut _start: u8;
    static mut _heap_start: u8;
}

impl MemoryMap {
    pub fn build_from_devicetree(dt: &FlattenedDeviceTree) -> MemoryMap {
        let kernel_area = unsafe {
            PhysicalAddrRange::from_start_end(
                PhysicalAddr::from_ptr(&raw const _start),
                PhysicalAddr::from_ptr(&raw const _heap_start),
            )
        };
        kdebug!("Kernel static area: {:?}", kernel_area);

        let root = dt.root().expect("Cannot read device tree root");
        let memory_nodes = root.children().named("memory");
        for memory_node in memory_nodes {
            let memory_area: PhysicalAddrRange = PhysicalAddrRange::from_reg(
                memory_node
                    .property("reg")
                    .expect("Memory device tree node does not have reg")
                    .reg()
                    .expect("Invaild memory reg property type"),
            );
            kdebug!("Memory area: {:?}", memory_area);
        }

        MemoryMap
    }
}
