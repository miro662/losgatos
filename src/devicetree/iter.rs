//! Helpers for iteration through device tree nodes

use super::node::NodeRef;

/// Extension trait for device tree nodes iterator
pub trait NodeIterExt<'dt> {
    /// Returns elements with given name
    fn named(self, name: &str) -> impl Iterator<Item = NodeRef<'dt>>;
}

impl<'dt, I: Iterator<Item = NodeRef<'dt>>> NodeIterExt<'dt> for I {
    fn named(self, name: &str) -> impl Iterator<Item = NodeRef<'dt>> {
        self.filter(move |n| n.name() == name)
    }
}
