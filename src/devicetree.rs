//! Zero-allocation DTB reader
//! 
//! This module provides a facility for reading [devicetrees](https://www.devicetree.org/).
//! They are descriptions of hardware available on a machine. They are available
//! in two formats - text format and device tree blob (DTB) - which is supported by this module.
//! 
//! This module is designed to be compliant with specification version [0.4](https://github.com/devicetree-org/devicetree-specification/releases/tag/v0.4).

pub mod error;
mod tokenizer;

use core::{ffi::c_void, iter, slice};

use alloc::string::String;

use crate::debug::kdebug;
use tokenizer::DeviceTreeTokenizer;

pub use error::DeviceTreeError;

/// Represents a reference to a device tree in a DTB format
pub struct DeviceTree(*const FdtHeader);

impl DeviceTree {
    /// DTB magic number. Checked during creating a reference.
    const DEVICE_TREE_MAGIC: u32 = 0xd00dfeed;

    /// Version of DTB supported by this module.
    const DEVICE_TREE_SUPPORTED_VERSION: u32 = 17;

    /// Loads correct device tree, depending on configuration
    ///     
    /// UNSAFE: if OVERRIDE_DTB is not specified
    /// this functions assumes that there is a vaild DTB header at `ptr`
    /// and that offsets and sizes defined in it are vaild and refer to available memory locations.
    #[cfg(not(OVERRIDE_DTB))]
    pub unsafe fn load_default(ptr: *const c_void) -> Result<DeviceTree, DeviceTreeError> {
        kdebug!("Loading SBI-provided devicetree from {:#x}", ptr as usize);
        Self::from_pointer(ptr)
    }

    /// Loads correct device tree, depending on configuration
    #[cfg(OVERRIDE_DTB)]
    pub unsafe fn load_default(ptr: *const c_void) -> DeviceTree {
        kdebug!(
            "Loading compile-time provided dtb from {}",
            env!("DTB_FILE")
        );
        let dtb = include_bytes!(env!("DTB_FILE"));
        Self::from_pointer(dtb);
    }

    /// Creates a DeviceTree reference to a memory location containing DTB.
    /// 
    /// This function checks whether device tree header contains a vaild header
    /// by checking its magic number. It also checks if its version is being supported
    /// by this module.
    /// 
    /// UNSAFE: this functions assumes that there is a vaild DTB header at `ptr`
    /// and that offsets and sizes defined in it are vaild and refer to available memory locations.
    pub unsafe fn from_pointer(ptr: *const c_void) -> Result<DeviceTree, DeviceTreeError> {
        let device_tree = DeviceTree(ptr as *const FdtHeader);

        let magic_number = device_tree.header().magic();
        if magic_number != Self::DEVICE_TREE_MAGIC {
            return Err(DeviceTreeError::InvaildMagicNumber {
                provided: magic_number,
                expected: Self::DEVICE_TREE_MAGIC,
            });
        };
        let version = device_tree.header().version();
        if version != Self::DEVICE_TREE_SUPPORTED_VERSION {
            return Err(DeviceTreeError::UnsupportedVersion {
                provided: version,
                expected: Self::DEVICE_TREE_SUPPORTED_VERSION,
            });
        };
        Ok(device_tree)
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

    fn tokenizer(&self) -> DeviceTreeTokenizer<'_> {
        DeviceTreeTokenizer {
            structure: self.header().dt_struct(self),
            strings: self.header().dt_string(self),
            position: 0,
        }
    }

    /// Prints a devicetree to kdebug
    pub fn print(&self) {
        let mut tokenizer = self.tokenizer();
        fn print_device_tree(tokenizer: &mut DeviceTreeTokenizer, tabs: usize) {
            let tabs_str: String = iter::repeat(' ').take(tabs).collect();
            loop {
                if let Some(name) = tokenizer.next_node().unwrap() {
                    kdebug!("{}{} {{", tabs_str, name);
                    print_device_tree(tokenizer, tabs + 4);
                    kdebug!("{}}}", tabs_str);
                } else if let Some((name, val)) = tokenizer.next_param().unwrap() {
                    kdebug!("{}{}: {:?};", tabs_str, name, val);
                } else if let Some(()) = tokenizer.next_node_end().unwrap() {
                    break;
                } else if tokenizer.is_eof().unwrap() {
                    break;
                } else {
                    panic!("Wrong format")
                }
            }
        }
        print_device_tree(&mut tokenizer, 0);
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