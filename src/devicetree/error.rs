use core::str::Utf8Error;

use snafu::prelude::*;

#[derive(Debug, Snafu)]
pub enum DeviceTreeError {
    #[snafu(display("Property has invaild size {actual} (expected {expected})"))]
    InvaildPropertySize { expected: usize, actual: usize },
    #[snafu(display("Invaild UTF-8 value: {source:?}"))]
    InvaildUTF8 { source: Utf8Error },
}
