#![no_std]

use core::fmt::Display;

use header::Header;
use identification::{Identification, EI_NIDENT};
use segment::Segment;

pub mod endiannes;
pub mod header;
pub mod identification;
pub mod machine;
pub mod segment;

/// Representation of an ELF file
pub struct Elf<'a> {
    identification: Identification<'a>,
    header: Header<'a>,
}

impl<'a> Elf<'a> {
    /// Reads this ELF file from bytes
    pub fn from_bytes(bytes: &'a [u8]) -> ElfResult<Elf<'a>> {
        let identification = Identification::from_bytes(bytes)?;
        if !identification.is_valid() {
            return Err(ElfError::InvaildIdentification);
        }

        let header = Header::from_bytes(&bytes[EI_NIDENT..], &identification)?;
        Ok(Elf {
            identification,
            header,
        })
    }

    pub fn segments(&'a self) -> impl Iterator<Item = Segment<'a>> {
        self.header.segments()
    }
}

impl<'a> Display for Elf<'a> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        writeln!(f, "{}\n{}", self.identification, self.header)?;
        writeln!(f, "segments:")?;
        for s in self.header.segments() {
            writeln!(f, "- {}", s)?;
        }
        Ok(())
    }
}

#[derive(Debug)]
pub enum ElfError {
    InvaildSize { minimal: usize, actual: usize },
    InvaildValue,
    InvaildIdentification,
}

impl core::fmt::Display for ElfError {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        // TODO: proper error display
        write!(f, "{:?}", self)
    }
}

impl core::error::Error for ElfError {}

pub type ElfResult<T> = Result<T, ElfError>;
