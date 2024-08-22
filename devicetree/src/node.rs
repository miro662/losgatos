//! Random access FDT parser types

use core::ffi::CStr;

use super::{
    error::DeviceTreeError,
    flattened::{FdtCell, FlattenedDeviceTree},
    value::DeviceTreeValue,
};

const FDT_BEGIN_NODE: FdtCell = 0x00000001;
const FDT_END_NODE: FdtCell = 0x00000002;
const FDT_PROP: FdtCell = 0x00000003;

/// Reference to a single node inside of a device tree
pub struct NodeRef<'dt> {
    fdt: &'dt FlattenedDeviceTree<'dt>,
    name: &'dt str,
    location: Option<usize>,
    data: &'dt [FdtCell],
    cell_sizes: CellSizes,
}

impl<'dt> NodeRef<'dt> {
    pub(super) fn from_slice(
        fdt: &'dt FlattenedDeviceTree<'dt>,
        slice: &'dt [FdtCell],
        cell_sizes: CellSizes,
    ) -> Result<NodeRef<'dt>, DeviceTreeError> {
        debug_assert_eq!(slice[0].to_be(), FDT_BEGIN_NODE);

        let bytes: &[u8] = unsafe { slice[1..].align_to().1 };
        let name_cstr = CStr::from_bytes_until_nul(bytes)
            .map_err(|_| DeviceTreeError::CStringConversionFail)?;
        let name_bytes_len = name_cstr.to_bytes().len();
        let name_words_len = name_bytes_len / 4 + 1;

        let name_str = name_cstr
            .to_str()
            .map_err(|source| DeviceTreeError::InvaildUTF8 { source })?;
        let mut name_it = name_str.split('@');
        let name = name_it.next().unwrap_or("");
        let location = name_it.next().and_then(|s| str::parse(s).ok());

        let data = &slice[name_words_len + 1..slice.len()];

        let mut node_ref = NodeRef {
            fdt,
            name,
            data,
            location,
            cell_sizes,
        };
        node_ref.cell_sizes = CellSizes::for_node(&node_ref, &node_ref.cell_sizes)?;

        Ok(node_ref)
    }

    /// Retrieves node's name
    pub fn name(&self) -> &str {
        self.name
    }

    /// Retrieves node's location, if specified {
    pub fn location(&self) -> Option<usize> {
        self.location
    }

    /// If node has given property, returns it
    pub fn property(&self, name: &str) -> Option<DeviceTreeValue> {
        self.properties().find(|(n, _)| *n == name).map(|(_, v)| v)
    }

    /// Returns an iterator returning a name-value pairs of node's properties
    pub fn properties(&'dt self) -> impl Iterator<Item = (&'dt str, DeviceTreeValue)> {
        PropertiesIterator { node: self, i: 0 }
    }

    /// If node has child with given name, return it
    pub fn child(&self, name: &str) -> Option<NodeRef> {
        self.children().find(|ch| ch.name() == name)
    }

    /// Returns an iterator iterating throguh all children nodes
    pub fn children(&'dt self) -> impl Iterator<Item = NodeRef<'dt>> {
        let mut i = 0;
        while i < self.data.len() {
            i += match self.data[i].to_be() {
                FDT_BEGIN_NODE => break,
                FDT_PROP => 3 + (3 + self.data[i + 1].to_be() as usize) / 4,
                _ => 1,
            };
        }

        NodesIterator { node: self, i }
    }

    fn cells(&self, name: &str, default: u32) -> Result<u32, DeviceTreeError> {
        if let Some(property) = self.property(name) {
            let value = property.u32()?;
            Ok(value)
        } else {
            Ok(default)
        }
    }
}

struct PropertiesIterator<'dt> {
    node: &'dt NodeRef<'dt>,
    i: usize,
}

impl<'dt> Iterator for PropertiesIterator<'dt> {
    type Item = (&'dt str, DeviceTreeValue<'dt>);

    fn next(&mut self) -> Option<Self::Item> {
        while self.i < self.node.data.len() {
            if self.node.data[self.i].to_be() == FDT_PROP {
                if self.i + 2 >= self.node.data.len() {
                    break;
                }
                let name_offset = self.node.data[self.i + 2].to_be() as usize;
                let name = self.node.fdt.string(name_offset).expect("Invaild name");

                let len = self.node.data[self.i + 1].to_be() as usize;
                let fdt_cell_len = len / 4;
                let value = &self.node.data[self.i + 3..self.i + 3 + fdt_cell_len];

                self.i += 3 + fdt_cell_len;
                return Some((
                    name,
                    DeviceTreeValue::wrap_cells(value, &self.node.cell_sizes),
                ));
            } else if self.node.data[self.i].to_be() == FDT_BEGIN_NODE {
                break;
            } else {
                self.i += 1;
            }
        }
        None
    }
}

struct NodesIterator<'dt> {
    node: &'dt NodeRef<'dt>,
    i: usize,
}

impl<'dt> Iterator for NodesIterator<'dt> {
    type Item = NodeRef<'dt>;

    fn next(&mut self) -> Option<Self::Item> {
        let mut stack = 0;
        if self.i >= self.node.data.len() {
            return None;
        }

        let mut start = self.i;
        while self.i < self.node.data.len() {
            match self.node.data[self.i].to_be() {
                FDT_BEGIN_NODE => {
                    if stack == 0 {
                        start = self.i;
                    }
                    self.i += 1;
                    stack += 1;
                }
                FDT_PROP => {
                    self.i += 3 + (3 + self.node.data[self.i + 1].to_be() as usize) / 4;
                }
                FDT_END_NODE => {
                    stack -= 1;
                    self.i += 1;
                    if stack == 0 {
                        return NodeRef::from_slice(
                            self.node.fdt,
                            &self.node.data[start..self.i],
                            self.node.cell_sizes,
                        )
                        .ok();
                    }
                }
                _ => self.i += 1,
            };
        }

        None
    }
}

#[derive(Debug, Clone, Copy)]
pub(crate) struct CellSizes {
    address: u32,
    size: u32,
}

impl CellSizes {
    pub(crate) fn address(&self) -> u32 {
        self.address
    }

    pub(crate) fn size(&self) -> u32 {
        self.size
    }

    fn for_node(node: &NodeRef, defaults: &CellSizes) -> Result<CellSizes, DeviceTreeError> {
        Ok(CellSizes {
            address: node.cells("#address-cells", defaults.address)?,
            size: node.cells("#size-cells", defaults.size)?,
        })
    }
}

impl Default for CellSizes {
    fn default() -> Self {
        Self {
            address: 2,
            size: 1,
        }
    }
}
