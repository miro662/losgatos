use core::{fmt::Display, ops::RangeInclusive};

use crate::{
    endiannes::Endianness,
    identification::{Class, Identification, EI_NIDENT},
    machine::Machine,
    segment::Segment,
    ElfResult,
};

pub struct Header<'a> {
    class: Class,
    endianess: Endianness,
    header_data: &'a [u8],
    data: &'a [u8],
}

impl<'a> Header<'a> {
    pub fn from_bytes(data: &'a [u8], identification: &Identification) -> ElfResult<Header<'a>> {
        let size = Self::size(identification.class()?);
        if data.len() < size {
            return Err(crate::ElfError::InvaildSize {
                minimal: size,
                actual: data.len(),
            });
        }

        let data_slice = &data[..size];
        Ok(Header {
            class: identification.class()?,
            endianess: identification.endiannes()?,
            header_data: data_slice,
            data: data,
        })
    }

    pub fn file_type(&'a self) -> FileType {
        let e_type = 0;
        match self.word(e_type) {
            1 => FileType::Relocatable,
            2 => FileType::Executable,
            3 => FileType::Shared,
            4 => FileType::Core,
            other => FileType::Other(other),
        }
    }

    pub fn machine(&'a self) -> Machine {
        let e_machine = 2;
        Machine::from_id(self.word(e_machine))
    }

    pub fn version(&self) -> u32 {
        let e_version = 4;
        self.dword(e_version)
    }

    pub fn entrypoint(&self) -> Option<usize> {
        let e_entry = 8;
        match self.addr(e_entry) {
            0 => None,
            other => Some(other),
        }
    }

    pub fn segments(&self) -> impl Iterator<Item = Segment> {
        self.segment_header_locations()
            .slices()
            .map(|s| Segment::from_bytes(&self.data[s], &self.data, self.class, self.endianess))
    }

    fn segment_header_locations(&self) -> HeaderLocations {
        let e_phoff = self.offset(8, 1);
        let e_phentsize = self.offset(14, 3);
        let e_phnum = self.offset(16, 3);

        HeaderLocations {
            offset: self.addr(e_phoff) - EI_NIDENT,
            size: self.word(e_phentsize),
            num: self.word(e_phnum),
        }
    }

    fn size(class: Class) -> usize {
        match class {
            Class::Class32 => 0x24,
            Class::Class64 => 0x30,
        }
    }

    fn word(&self, offset: usize) -> u16 {
        self.endianess
            .read_u16(self.header_data[offset..offset + 2].try_into().unwrap())
    }

    fn dword(&self, offset: usize) -> u32 {
        self.endianess
            .read_u32(self.header_data[offset..offset + 4].try_into().unwrap())
    }

    fn qword(&self, offset: usize) -> u64 {
        self.endianess
            .read_u64(self.header_data[offset..offset + 8].try_into().unwrap())
    }

    fn addr(&self, offset: usize) -> usize {
        match self.class {
            Class::Class32 => self.dword(offset) as usize,
            Class::Class64 => self.qword(offset) as usize,
        }
    }

    fn offset(&self, bytes: usize, addr_fields: usize) -> usize {
        bytes
            + match self.class {
                Class::Class32 => 4,
                Class::Class64 => 8,
            } * addr_fields
    }
}

impl<'a> Display for Header<'a> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(
            f,
            "{}, {} v{}",
            self.machine(),
            self.file_type(),
            self.version()
        )?;
        if let Some(entrypoint_addr) = self.entrypoint() {
            write!(f, "\nentrypoint: 0x{:x}", entrypoint_addr)?;
        }
        Ok(())
    }
}

pub enum FileType {
    Relocatable,
    Executable,
    Shared,
    Core,
    Other(u16),
}

impl<'a> Display for FileType {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        match self {
            Self::Relocatable => write!(f, "relocatable"),
            Self::Executable => write!(f, "executable"),
            Self::Shared => write!(f, "shared"),
            Self::Core => write!(f, "core"),
            Self::Other(value) => write!(f, "unknown type 0x{:x}", value),
        }
    }
}

#[derive(Debug)]
struct HeaderLocations {
    offset: usize,
    size: u16,
    num: u16,
}

impl HeaderLocations {
    fn slice_loc(&self, i: u16) -> usize {
        self.offset + (self.size * i) as usize
    }

    fn slices(self) -> impl Iterator<Item = RangeInclusive<usize>> {
        (0..self.num).map(move |i| self.slice_loc(i)..=self.slice_loc(i + 1))
    }
}
