#![no_std]
//! no_std library for device tree retrieval.
//!
//! Designed to be used during very early retrieval stage. Does not require alloc.
//! However, this has a drawback of relatively high computational complexity.

mod error;
mod flattened;
mod iter;
mod node;
mod value;

pub use error::DeviceTreeError;
pub use flattened::{FdtHeader, FlattenedDeviceTree};
pub use iter::NodeIterExt;
pub use node::NodeRef;
pub use value::DeviceTreeValue;
