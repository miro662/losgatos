use core::str;

use crate::node::CellSizes;

use super::{error::DeviceTreeError, flattened::FdtCell};

pub struct DeviceTreeValue<'dt>(&'dt [FdtCell], &'dt CellSizes);

impl<'dt> DeviceTreeValue<'dt> {
    pub(crate) fn wrap_cells(
        cells: &'dt [FdtCell],
        cell_sizes: &'dt CellSizes,
    ) -> DeviceTreeValue<'dt> {
        DeviceTreeValue(cells, cell_sizes)
    }

    pub fn u32(&self) -> Result<u32, DeviceTreeError> {
        self.try_into().map(u32::to_be)
    }

    pub fn string(&self) -> Result<&str, DeviceTreeError> {
        self.try_into()
    }

    pub fn reg(&self) -> Result<(usize, usize), DeviceTreeError> {
        let address_cells = self.1.address() as usize;
        let size_cells = self.1.size() as usize;
        self.expect_size(address_cells + size_cells)?;

        let address = self.read_from_cells(0, address_cells);
        let size = self.read_from_cells(address_cells, size_cells);

        Ok((address, size))
    }

    fn expect_size(&self, expected: usize) -> Result<(), DeviceTreeError> {
        if self.0.len() != expected {
            Err(DeviceTreeError::InvaildPropertySize {
                expected,
                actual: self.0.len(),
            })
        } else {
            Ok(())
        }
    }

    fn read_from_cells(&self, start: usize, cells: usize) -> usize {
        let mut result = 0;
        for i in start..start + cells {
            result += self.0[i].to_be() as usize;
            if i != start + cells - 1 {
                result <<= 32;
            }
        }
        result
    }
}

impl<'a: 'dt, 'dt> TryFrom<&'a DeviceTreeValue<'dt>> for u32 {
    type Error = DeviceTreeError;

    fn try_from(DeviceTreeValue(value, _): &'a DeviceTreeValue<'dt>) -> Result<Self, Self::Error> {
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

    fn try_from(DeviceTreeValue(value, _): &'a DeviceTreeValue<'dt>) -> Result<Self, Self::Error> {
        // SAFETY: 32 is divisible by 8
        let (_, bytes, _) = unsafe { value.align_to::<u8>() };
        str::from_utf8(bytes).map_err(|e| DeviceTreeError::InvaildUTF8 { source: e })
    }
}
