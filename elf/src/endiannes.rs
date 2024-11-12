use core::fmt::Display;

#[derive(Debug, Hash, PartialEq, Eq, Clone, Copy)]
pub enum Endianness {
    BigEndian,
    LittleEndian,
}

impl Endianness {
    pub fn read_u16(&self, bytes: [u8; 2]) -> u16 {
        match self {
            Endianness::BigEndian => u16::from_be_bytes(bytes),
            Endianness::LittleEndian => u16::from_le_bytes(bytes),
        }
    }

    pub fn read_u32(&self, bytes: [u8; 4]) -> u32 {
        match self {
            Endianness::BigEndian => u32::from_be_bytes(bytes),
            Endianness::LittleEndian => u32::from_le_bytes(bytes),
        }
    }

    pub fn read_u64(&self, bytes: [u8; 8]) -> u64 {
        match self {
            Endianness::BigEndian => u64::from_be_bytes(bytes),
            Endianness::LittleEndian => u64::from_le_bytes(bytes),
        }
    }
}

impl Display for Endianness {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        match self {
            Endianness::BigEndian => write!(f, "big-endian"),
            Endianness::LittleEndian => write!(f, "little-endian"),
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::endiannes::Endianness;

    #[test]
    fn test_read_u16() {
        let value = [0x01, 0x02];
        assert_eq!(Endianness::BigEndian.read_u16(value), 0x102);
        assert_eq!(Endianness::LittleEndian.read_u16(value), 0x201);
    }

    #[test]
    fn test_read_u32() {
        let value = [0x01, 0x02, 0x03, 0x04];
        assert_eq!(Endianness::BigEndian.read_u32(value), 0x1020304);
        assert_eq!(Endianness::LittleEndian.read_u32(value), 0x4030201);
    }

    #[test]
    fn test_read_u64() {
        let value = [0x01, 0x02, 0x03, 0x04, 0x05, 0x06, 0x07, 0x08];
        assert_eq!(Endianness::BigEndian.read_u64(value), 0x102030405060708);
        assert_eq!(Endianness::LittleEndian.read_u64(value), 0x080706054030201);
    }
}
