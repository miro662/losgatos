//! Random access FDT parser types

use core::ffi::CStr;

use super::{
    flattened::{FdtCell, FlattenedDeviceTree},
    value::DeviceTreeValue,
};

const FDT_BEGIN_NODE: FdtCell = 0x00000001;
const FDT_END_NODE: FdtCell = 0x00000002;
const FDT_PROP: FdtCell = 0x00000003;
const FDT_NOP: FdtCell = 0x00000004;

/// Reference to a single node inside of a device tree
pub struct NodeRef<'dt> {
    fdt: &'dt FlattenedDeviceTree<'dt>,
    name: &'dt CStr,
    data: &'dt [FdtCell],
}

impl<'dt> NodeRef<'dt> {
    pub(super) fn from_slice(
        fdt: &'dt FlattenedDeviceTree<'dt>,
        slice: &'dt [FdtCell],
    ) -> NodeRef<'dt> {
        debug_assert_eq!(slice[0].to_be(), FDT_BEGIN_NODE);
        // debug_assert_eq!(slice.last().unwrap().to_be(), FDT_END_NODE);

        let bytes: &[u8] = unsafe { slice[1..].align_to().1 };
        let name = CStr::from_bytes_until_nul(bytes).expect("Invaild name");
        let name_bytes_len = name.to_bytes().len();
        let name_words_len = name_bytes_len / 4 + 1;

        let data = &slice[name_words_len + 1..slice.len()];

        NodeRef { fdt, name, data }
    }

    /// Retrieves node's name
    pub fn name(&self) -> &str {
        self.name.to_str().expect("Invaild UTF-8")
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
                let FdtCell_len = len / 4;
                let value = &self.node.data[self.i + 3..self.i + 3 + FdtCell_len];

                self.i += 3 + FdtCell_len;
                return Some((name, value.into()));
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
                        return Some(NodeRef::from_slice(
                            self.node.fdt,
                            &self.node.data[start..self.i],
                        ));
                    }
                }
                _ => self.i += 1,
            };
        }

        None
    }
}
