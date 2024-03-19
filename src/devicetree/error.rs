use core::str::Utf8Error;

/// Error type used for [devicetree] module.
#[derive(Debug)]
pub enum DeviceTreeError {
    /// DTB header contains invalid magic number.
    InvaildMagicNumber { provided: u32, expected: u32 },
    /// Version specified in DTB header is not supported by this module.
    UnsupportedVersion { provided: u32, expected: u32 },
    /// Unexpected DTB token has been spotted.
    UnexpectedToken { token: u32, position: usize },
    /// Parser attempted to read data outside of bounds of structure part of DTB, defined in header.
    OutOfStructureBounds,
    /// An error occured during parsing a byte slice.
    BytesParseError,
    /// Cannot find a `\0` in an expected null-terminated string.
    UnterminatedString,
    /// A string is not a vaild UTF-8 sequence.
    InvaildUTF8(Utf8Error),
}