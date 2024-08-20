use core::str;

use super::{error::DeviceTreeError, flattened::FdtCell};

pub struct DeviceTreeValue<'dt>(&'dt [FdtCell]);

impl<'dt> DeviceTreeValue<'dt> {
    pub fn u32(&self) -> Result<u32, DeviceTreeError> {
        self.try_into()
    }

    pub fn string(&self) -> Result<&str, DeviceTreeError> {
        self.try_into()
    }
}

impl<'a: 'dt, 'dt> TryFrom<&'a DeviceTreeValue<'dt>> for u32 {
    type Error = DeviceTreeError;

    fn try_from(DeviceTreeValue(value): &'a DeviceTreeValue<'dt>) -> Result<Self, Self::Error> {
        if value.len() != 1 {
            Err(DeviceTreeError::InvaildPropertySize {
                expected: 1,
                actual: value.len(),
            })
        } else {
            Ok(value[0])
        }
    }
}

impl<'a: 'dt, 'dt> TryFrom<&'a DeviceTreeValue<'dt>> for &'dt str {
    type Error = DeviceTreeError;

    fn try_from(DeviceTreeValue(value): &'a DeviceTreeValue<'dt>) -> Result<Self, Self::Error> {
        // SAFETY: 32 is divisible by 8
        let (_, bytes, _) = unsafe { value.align_to::<u8>() };
        str::from_utf8(bytes).map_err(|e| DeviceTreeError::InvaildUTF8 { source: e })
    }
}

impl<'dt> From<&'dt [FdtCell]> for DeviceTreeValue<'dt> {
    fn from(value: &'dt [FdtCell]) -> Self {
        DeviceTreeValue(value)
    }
}
