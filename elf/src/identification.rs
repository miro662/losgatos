use core::fmt::Display;

use crate::{endiannes::Endianness, ElfError, ElfResult};

/// Indentification section length
pub const EI_NIDENT: usize = 16;

/// Indentification section
#[derive(Debug)]
pub struct Identification<'a>(&'a [u8; EI_NIDENT]);

impl<'a> Identification<'a> {
    const MAG: [u8; 4] = [0x7f, 'E' as u8, 'L' as u8, 'F' as u8];

    const EI_MAG0: usize = 0;
    const EI_MAG3: usize = 3;
    const EI_CLASS: usize = 4;
    const EI_DATA: usize = 5;
    const EI_VERSION: usize = 6;

    /// Reads identification section from first bytes of provided slice
    pub fn from_bytes(bytes: &'a [u8]) -> ElfResult<Identification<'a>> {
        if bytes.len() < EI_NIDENT {
            Err(ElfError::InvaildSize {
                minimal: EI_NIDENT,
                actual: bytes.len(),
            })
        } else {
            Ok(Identification(bytes[0..EI_NIDENT].try_into().unwrap()))
        }
    }

    /// Checks if magic number is vaild
    pub fn is_valid(&'a self) -> bool {
        self.magic_number() == &Self::MAG && self.version() == 1
    }

    /// File's class - whether it is 32-bit or 64-bit
    pub fn class(&'a self) -> ElfResult<Class> {
        match self.0[Self::EI_CLASS] {
            1 => Ok(Class::Class32),
            2 => Ok(Class::Class64),
            _ => Err(ElfError::InvaildValue),
        }
    }

    /// File's endiannes
    pub fn endiannes(&'a self) -> ElfResult<Endianness> {
        match self.0[Self::EI_DATA] {
            1 => Ok(Endianness::LittleEndian),
            2 => Ok(Endianness::BigEndian),
            _ => Err(ElfError::InvaildValue),
        }
    }

    /// ELF version, expected to be 1
    pub fn version(&'a self) -> u8 {
        self.0[Self::EI_VERSION]
    }

    fn magic_number(&'a self) -> &'a [u8] {
        &self.0[Self::EI_MAG0..=Self::EI_MAG3]
    }
}

impl<'a> Display for Identification<'a> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        use ResultDisplay as R;

        if self.is_valid() {
            write!(f, "{}, {} ELF file", R(self.class()), R(self.endiannes()))?;
            Ok(())
        } else {
            write!(f, "Invaild ELF file")
        }
    }
}

#[derive(Debug, Hash, PartialEq, Eq, Clone, Copy)]
pub enum Class {
    Class32,
    Class64,
}

impl Display for Class {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        match self {
            Class::Class32 => write!(f, "32-bit"),
            Class::Class64 => write!(f, "64-bit"),
        }
    }
}
struct ResultDisplay<T>(ElfResult<T>);

impl<T: Display> Display for ResultDisplay<T> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        match &self.0 {
            Ok(s) => write!(f, "{}", s),
            Err(_) => write!(f, "?"),
        }
    }
}
