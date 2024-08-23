use core::iter::once;

use devicetree::{DeviceTreeError, NodeIterExt, NodeRef};

use crate::{
    data_structures::CapacityVec, debug::kdebug, memory::static_area::kernel_static_memory_area,
};

use super::MemoryRange;

const MAX_MEMORY_MAP_ENTRIES: usize = 64;

pub struct MemoryMap {
    available_memory_areas: CapacityVec<MemoryRange, MAX_MEMORY_MAP_ENTRIES>,
}

impl MemoryMap {
    pub fn from_device_tree(dt_root: &NodeRef) -> Result<MemoryMap, DeviceTreeError> {
        let memory_nodes = dt_root.children().named("memory");
        let mut memory_areas: CapacityVec<MemoryRange, MAX_MEMORY_MAP_ENTRIES> = memory_nodes
            .take(MAX_MEMORY_MAP_ENTRIES)
            .filter_map(|node: NodeRef| {
                node.property("reg").map(|r| r.reg().map(MemoryRange::from))
            })
            .collect::<Result<_, _>>()?;
        memory_areas.sort_unstable_by_key(|a| a.address());
        for ma in memory_areas.iter() {
            kdebug!("MA: {:?}", ma);
        }

        let kernel_memory = kernel_static_memory_area();
        let mut reserved_memory_areas: CapacityVec<MemoryRange, 32> = once(kernel_memory).collect();
        if let Some(reserved) = dt_root.child("reserved-memory") {
            for child in reserved.children().take(32) {
                if let Some(reg) = child.property("reg") {
                    let range = reg.reg()?.into();
                    reserved_memory_areas.push(range);
                }
            }
        }
        reserved_memory_areas.sort_unstable_by_key(|a| a.address());
        for rma in reserved_memory_areas.iter() {
            kdebug!("RMA: {:?}", rma);
        }

        let mut available_memory_areas: CapacityVec<MemoryRange, MAX_MEMORY_MAP_ENTRIES> =
            CapacityVec::empty();
        for available_area in memory_areas {
            let mut remaining_area = Some(available_area);
            'reserved_loop: for rma in reserved_memory_areas.iter() {
                let Some(ra) = remaining_area else {
                    break 'reserved_loop;
                };
                if rma.address() >= ra.address() || rma.address() <= ra.end_address() {
                    let new_end = rma.address() - 1;
                    if new_end > ra.address() {
                        let area_before = MemoryRange::from_start_and_end(ra.address(), new_end);
                        available_memory_areas.push(area_before);
                    }

                    let remaining_area_start = (rma.end_address() + 1).max(ra.address());
                    remaining_area = if remaining_area_start < ra.end_address() {
                        Some(MemoryRange::from_start_and_end(
                            remaining_area_start,
                            ra.end_address(),
                        ))
                    } else {
                        None
                    };
                }
            }

            if let Some(ra) = remaining_area {
                available_memory_areas.push(ra);
            }
        }

        Ok(MemoryMap {
            available_memory_areas,
        })
    }

    pub fn describe(&self) {
        kdebug!("Available memory areas:");
        for area in self.available_areas() {
            kdebug!(" - {:?}", area);
        }
        kdebug!("Total available size: {} bytes", self.total_size())
    }

    pub fn total_size(&self) -> usize {
        self.available_areas().iter().map(|a| a.size()).sum()
    }

    pub fn available_areas(&self) -> &[MemoryRange] {
        &self.available_memory_areas
    }

    pub fn is_page_aligned(&self) -> bool {
        self.available_areas()
            .iter()
            .all(MemoryRange::is_page_aligned)
    }
}
