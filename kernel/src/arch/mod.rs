#[cfg(target_arch = "riscv64")]
mod riscv64;
#[cfg(target_arch = "riscv64")]
pub use riscv64::*;

/// Abstraction for a CPU-dependent functions used by kernel
pub unsafe trait Arch {
    const PAGE_SIZE: usize;

    /// Hangs hardware thread until an interrupt will be received
    fn wfi() -> !;

    /// Prints a byte on a debug output
    unsafe fn putc(byte: u8);
}
