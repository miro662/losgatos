use core::{fmt::Debug, ptr::addr_of};

#[derive(Copy, Clone, Hash, PartialEq, Eq, PartialOrd, Ord)]
#[repr(transparent)]
pub struct PhysicalAddr(usize);

impl Debug for PhysicalAddr {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "[0x{:x}]", self.0)
    }
}

impl PhysicalAddr {
    pub unsafe fn from_ptr<T>(ptr: *const T) -> PhysicalAddr {
        PhysicalAddr(ptr as usize)
    }
}

#[derive(Copy, Clone, Hash, PartialEq, Eq)]
pub struct PhysicalAddrRange {
    addr: PhysicalAddr,
    size: usize,
}

impl PhysicalAddrRange {
    pub fn from_reg((addr, size): (usize, usize)) -> PhysicalAddrRange {
        PhysicalAddrRange {
            addr: PhysicalAddr(addr),
            size,
        }
    }

    pub fn from_start_end(start: PhysicalAddr, end: PhysicalAddr) -> PhysicalAddrRange {
        if end < start {
            panic!("Physical address range end cannot be greater than start")
        }

        PhysicalAddrRange {
            addr: start,
            size: end.0 - start.0 + 1,
        }
    }

    pub fn start(&self) -> PhysicalAddr {
        self.addr
    }

    pub fn end(&self) -> PhysicalAddr {
        PhysicalAddr(self.addr.0 + self.size - 1)
    }

    pub fn size(&self) -> usize {
        self.size
    }
}

impl Debug for PhysicalAddrRange {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "[{:?}-{:?}, {} bytes]", self.start(), self.end(), self.size())
    }
}