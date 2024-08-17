//! FDT binary representation types

use core::{ffi::CStr, slice};

use crate::debug::kdebug;

use super::node::NodeRef;

#[derive(Debug)]
/// A structure containing references to elements of a device tree
pub struct FlattenedDeviceTree<'dt> {
    header: &'dt DeviceTreeHeader,
    structure: &'dt [u32],
    strings: &'dt [u8],
}

impl<'dt> FlattenedDeviceTree<'dt> {
    /// Interprets given pointer loaction as a beginning of a device tree.
    ///
    /// Loads a device tree header from given location.
    /// Structure and strings references are based on `{size/off}_dt_{struct/strings}` fields of the header
    ///
    /// If pointer does not point to a device tree header (determined by its magic number)
    ///     or is a null reference, function returns `None`
    ///
    /// SAFETY: this function assumes that pointer points to a vaild device tree.
    pub unsafe fn from_ptr(ptr: *const DeviceTreeHeader) -> Option<FlattenedDeviceTree<'dt>> {
        let header = ptr.as_ref()?;
        if header.is_correct() {
            let fdt = FlattenedDeviceTree {
                header,
                structure: &Self::offset_and_size_to_slice(
                    ptr,
                    header.off_dt_struct(),
                    header.size_dt_struct(),
                ),
                strings: Self::offset_and_size_to_slice(
                    ptr,
                    header.off_dt_strings(),
                    header.size_dt_strings(),
                ),
            };
            Some(fdt)
        } else {
            None
        }
    }

    /// Returns a reference to a device tree's root node
    pub fn root(&self) -> NodeRef {
        const FDT_END: u32 = 0x00000009;
        debug_assert_eq!(self.structure[self.structure.len() - 1].to_be(), FDT_END);
        NodeRef::from_slice(&self, &self.structure[0..self.structure.len() - 2])
    }

    pub(super) fn string(&self, offset: usize) -> Option<&str> {
        CStr::from_bytes_until_nul(&self.strings[offset..])
            .ok()
            .and_then(|s| s.to_str().ok())
    }

    fn header(&self) -> &DeviceTreeHeader {
        &self.header
    }

    unsafe fn offset_and_size_to_slice<'a, A, T>(ptr: *const A, offset: u32, size: u32) -> &'a [T] {
        let offset_ptr = ptr.byte_offset(offset as isize) as *const T;
        slice::from_raw_parts(offset_ptr, (size as usize) / size_of::<T>())
    }
}

/// Device tree header, as defined in a devicetree standard
#[repr(C)]
#[derive(Debug)]
pub struct DeviceTreeHeader {
    magic: u32,
    totalsize: u32,
    off_dt_struct: u32,
    off_dt_strings: u32,
    off_mem_rsvmap: u32,
    version: u32,
    last_comp_version: u32,
    boot_cpuid_phys: u32,
    size_dt_strings: u32,
    size_dt_struct: u32,
}

impl DeviceTreeHeader {
    const MAGIC_NUMBER: u32 = 0xd00dfeed;

    /// Magic number, in host byte order
    pub fn magic(&self) -> u32 {
        self.magic.to_be()
    }

    /// Check if magic number is correct (equal to)
    pub fn is_correct(&self) -> bool {
        self.magic() == Self::MAGIC_NUMBER
    }

    /// Returns offset to a struct data (in host byte order)
    pub fn off_dt_struct(&self) -> u32 {
        self.off_dt_struct.to_be()
    }

    /// Returns offset to strings data (in host byte order)
    pub fn off_dt_strings(&self) -> u32 {
        self.off_dt_strings.to_be()
    }

    /// Returns size of a struct data (in host byte order)
    pub fn size_dt_struct(&self) -> u32 {
        self.size_dt_struct.to_be()
    }

    /// Returns size of strings data (in host byte order)
    pub fn size_dt_strings(&self) -> u32 {
        self.size_dt_strings.to_be()
    }
}
