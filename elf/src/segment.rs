use bitflags::bitflags;
use core::fmt::Display;

use crate::{endiannes::Endianness, identification::Class};

pub struct Segment<'a> {
    segment_data: &'a [u8],
    data: &'a [u8],
    class: Class,
    endianness: Endianness,
}

impl<'a> Segment<'a> {
    pub fn from_bytes(
        segment_data: &'a [u8],
        data: &'a [u8],
        class: Class,
        endianness: Endianness,
    ) -> Segment<'a> {
        Segment {
            segment_data,
            data,
            class,
            endianness,
        }
    }

    pub fn segment_type(&self) -> SegmentType {
        match self.read_u32(0, 0) {
            0 => SegmentType::Unused,
            1 => SegmentType::Load,
            2 => SegmentType::Dynamic,
            3 => SegmentType::Interpreter,
            4 => SegmentType::Note,
            6 => SegmentType::ProgramHeader,
            7 => SegmentType::TLS,
            other => SegmentType::Other(other),
        }
    }

    pub fn vaddr(&self) -> usize {
        self.read_usize(0x08, 0x10)
    }

    pub fn paddr(&self) -> usize {
        self.read_usize(0x0c, 0x18)
    }

    pub fn memory_size(&self) -> usize {
        self.read_usize(0x14, 0x20)
    }

    pub fn executable(&self) -> bool {
        self.flags().contains(SegmentFlags::EXECUTABLE)
    }

    pub fn writeable(&self) -> bool {
        self.flags().contains(SegmentFlags::WRITEABLE)
    }

    pub fn readable(&self) -> bool {
        self.flags().contains(SegmentFlags::READABLE)
    }

    pub fn data(&self) -> &'a [u8] {
        let offset = self.read_usize(0x04, 0x08);
        let file_size = self.read_usize(0x10, 0x20);
        &self.data[offset..offset + file_size]
    }

    fn flags(&self) -> SegmentFlags {
        SegmentFlags::from_bits_retain(self.read_u32(0x18, 0x04))
    }

    fn read_u32(&self, loc_32: usize, loc_64: usize) -> u32 {
        let loc = match self.class {
            Class::Class32 => loc_32,
            Class::Class64 => loc_64,
        };
        self.endianness
            .read_u32(self.segment_data[loc..loc + 4].try_into().unwrap())
    }
    fn read_usize(&self, loc_32: usize, loc_64: usize) -> usize {
        match self.class {
            Class::Class32 => self
                .endianness
                .read_u32(self.segment_data[loc_32..loc_32 + 4].try_into().unwrap())
                as usize,
            Class::Class64 => self
                .endianness
                .read_u64(self.segment_data[loc_64..loc_64 + 8].try_into().unwrap())
                as usize,
        }
    }
}

impl<'a> Display for Segment<'a> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        writeln!(
            f,
            "{} {} size: {:#x}",
            self.segment_type(),
            self.flags(),
            self.data().len()
        )?;
        write!(
            f,
            "-> vaddr: {:#016x}, paddr: {:#016x}, memory_size: {:#x}",
            self.vaddr(),
            self.paddr(),
            self.memory_size()
        )
    }
}

pub enum SegmentType {
    Unused,
    Load,
    Dynamic,
    Interpreter,
    Note,
    ProgramHeader,
    TLS,
    Other(u32),
}

impl Display for SegmentType {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        match self {
            Self::Unused => write!(f, "NULL"),
            Self::Load => write!(f, "LOAD"),
            Self::Dynamic => write!(f, "DYNAMIC"),
            Self::Interpreter => write!(f, "INTERP"),
            Self::Note => write!(f, "NOTE"),
            Self::ProgramHeader => write!(f, "PHDR"),
            Self::TLS => write!(f, "TLS"),
            Self::Other(other) => write!(f, "UNKNOWN 0x{:x}", other),
        }
    }
}

bitflags! {
    struct SegmentFlags: u32 {
        const EXECUTABLE = 0x1;
        const WRITEABLE = 0x2;
        const READABLE = 0x4;
    }
}

impl Display for SegmentFlags {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        fn flag(
            f: &mut core::fmt::Formatter<'_>,
            letter: char,
            flag: &SegmentFlags,
            expected: SegmentFlags,
        ) -> core::fmt::Result {
            if flag.contains(expected) {
                write!(f, "{}", letter)
            } else {
                write!(f, "-")
            }
        }

        flag(f, 'r', self, SegmentFlags::READABLE)?;
        flag(f, 'w', self, SegmentFlags::WRITEABLE)?;
        flag(f, 'x', self, SegmentFlags::EXECUTABLE)
    }
}
