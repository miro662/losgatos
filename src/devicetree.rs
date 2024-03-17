use core::{ffi::c_void, slice};

use crate::debug::kdebug;

/// A reference to a device tree
pub struct DeviceTree(*const FdtHeader);

impl DeviceTree {
    const DEVICE_TREE_MAGIC: u32 = 0xd00dfeed;
    const DEVICE_TREE_SUPPORTED_VERSION: u32 = 17;

    /// Creates a DeviceTree reference from a pointer
    pub fn from_pointer(ptr: *const c_void) -> DeviceTree {
        let device_tree = DeviceTree(ptr as *const FdtHeader);
        if device_tree.header().magic() != Self::DEVICE_TREE_MAGIC {
            panic!(
                "Invaild device tree magic number: {:#x}",
                device_tree.header().magic()
            )
        };
        if device_tree.header().version() != Self::DEVICE_TREE_SUPPORTED_VERSION {
            panic!(
                "Unsupported device tree version: {:#x}",
                device_tree.header().version()
            )
        };
        kdebug!("{:?}", device_tree.header().dt_struct(&device_tree));
        device_tree
    }

    fn header(&self) -> &FdtHeader {
        unsafe { self.0.as_ref().unwrap() }
    }

    /// Returns a list of memory regions that should not be used
    pub fn reserved_entries(&self) -> ReserveEntryIterator {
        ReserveEntryIterator {
            current_item: self.header().mem_rsvmap(self),
        }
    }
}

/// Device tree header
#[derive(Debug)]
#[repr(C)]
struct FdtHeader {
    magic: u32,
    totalsize: u32,
    off_dt_struct: u32,
    off_dt_string: u32,
    off_mem_rsvmap: u32,
    version: u32,
    last_comp_version: u32,
    boot_cpuid_phys: u32,
    size_dt_strings: u32,
    size_dt_struct: u32,
}

impl FdtHeader {
    fn magic(&self) -> u32 {
        self.magic.to_be()
    }

    fn version(&self) -> u32 {
        self.version.to_be()
    }

    fn mem_rsvmap(&self, start_ptr: &DeviceTree) -> *const ReserveEntry {
        let start_ptr = start_ptr.0 as *const u8;
        start_ptr.wrapping_add(self.off_mem_rsvmap.to_be() as usize) as *const ReserveEntry
    }

    fn dt_struct(&self, start_ptr: &DeviceTree) -> &[u8] {
        let start_ptr = start_ptr.0 as *const u8;
        unsafe {
            slice::from_raw_parts(
                start_ptr.add(self.off_dt_struct.to_be() as usize),
                self.size_dt_struct.to_be() as usize,
            )
        }
    }

    fn dt_string(&self, start_ptr: &DeviceTree) -> &[u8] {
        let start_ptr = start_ptr.0 as *const u8;
        unsafe {
            slice::from_raw_parts(
                start_ptr.add(self.off_dt_string.to_be() as usize),
                self.size_dt_strings.to_be() as usize,
            )
        }
    }
}

/// Represents an area of memory that should not be used
#[derive(Debug, Copy, Clone)]
#[repr(C)]
pub struct ReserveEntry {
    address: u64,
    size: u64,
}

/// Iterates through all reserved entries
pub struct ReserveEntryIterator {
    current_item: *const ReserveEntry,
}

impl Iterator for ReserveEntryIterator {
    type Item = ReserveEntry;

    fn next(&mut self) -> Option<Self::Item> {
        let item = unsafe { self.current_item.as_ref().unwrap() };
        if item.address == 0 && item.size == 0 {
            None
        } else {
            let result = Some(*item);
            self.current_item = self.current_item.wrapping_add(1);
            result
        }
    }
}
