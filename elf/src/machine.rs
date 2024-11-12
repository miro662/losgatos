use core::fmt::Display;

#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq)]
pub struct Machine(u16);

impl Machine {
    pub fn from_id(id: u16) -> Machine {
        Machine(id)
    }
}

impl Display for Machine {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        match self.0 {
            0x03 => write!(f, "x86"),
            0x3e => write!(f, "x86_64"),
            0x28 => write!(f, "ARM"),
            0xb7 => write!(f, "AArch64"),
            0xf3 => write!(f, "RISC-V"),
            other => write!(f, "unknown machine 0x{:x}", other),
        }
    }
}
