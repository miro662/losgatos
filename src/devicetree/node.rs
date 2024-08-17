//! Random access FDT parser types

use core::{ffi::CStr, mem::transmute};

use super::flattened::FlattenedDeviceTree;

const FDT_BEGIN_NODE: u32 = 0x00000001;
const FDT_END_NODE: u32 = 0x00000002;
const FDT_PROP: u32 = 0x00000003;

/// Reference to a single node inside of a device tree
pub struct NodeRef<'dt> {
    fdt: &'dt FlattenedDeviceTree<'dt>,
    name: &'dt CStr,
    data: &'dt [u32],
}

impl<'dt> NodeRef<'dt> {
    pub(super) fn from_slice(
        fdt: &'dt FlattenedDeviceTree<'dt>,
        slice: &'dt [u32],
    ) -> NodeRef<'dt> {
        debug_assert_eq!(slice[0].to_be(), FDT_BEGIN_NODE);
        debug_assert_eq!(slice.last().unwrap().to_be(), FDT_END_NODE);

        let bytes: &[u8] = unsafe { transmute(&slice[1..]) };
        let name = CStr::from_bytes_until_nul(bytes).expect("Invaild name");
        let name_bytes_len = name.to_bytes().len();
        let name_words_len = name_bytes_len / 4 + 1;

        let data = &slice[name_words_len + 1..slice.len() - 2];

        NodeRef { fdt, name, data }
    }

    /// Retrieves node's name
    pub fn name(&self) -> &str {
        self.name.to_str().expect("Invaild UTF-8")
    }

    /// If node has given property, returns it
    pub fn property(&self, name: &str) -> Option<&[u32]> {
        self.properties().find(|(n, _)| *n == name).map(|(_, v)| v)
    }

    /// Returns an iterator returning a name-value pairs of node's properties
    pub fn properties(&'dt self) -> impl Iterator<Item = (&'dt str, &'dt [u32])> {
        PropertiesIterator { node: &self, i: 0 }
    }
}

struct PropertiesIterator<'dt> {
    node: &'dt NodeRef<'dt>,
    i: usize,
}

impl<'dt> Iterator for PropertiesIterator<'dt> {
    type Item = (&'dt str, &'dt [u32]);

    fn next(&mut self) -> Option<Self::Item> {
        while self.i < self.node.data.len() {
            if self.node.data[self.i].to_be() == FDT_PROP {
                let name_offset = self.node.data[self.i + 2].to_be() as usize;
                let name = self.node.fdt.string(name_offset).expect("Invaild name");

                let len = self.node.data[self.i + 1].to_be() as usize;
                let u32_len = len as usize / 4;
                let value = &self.node.data[self.i + 2..self.i + 2 + u32_len];

                self.i += 3 + u32_len;
                return Some((name, value));
            } else if self.node.data[self.i].to_be() == FDT_BEGIN_NODE {
                break;
            } else {
                self.i += 1;
            }
        }
        None
    }
}
