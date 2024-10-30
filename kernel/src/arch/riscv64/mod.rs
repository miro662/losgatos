mod entry;
mod sbi;

use core::arch::asm;

use super::Arch;

pub const PAGE_SIZE: usize = 4096;

struct Riscv64;

unsafe impl Arch for Riscv64 {
    const PAGE_SIZE: usize = 4096;

    fn wfi() -> ! {
        // safety: this instruction hangs processor until an interrupt is received
        unsafe { asm!("wfi") }
        #[allow(clippy::empty_loop)]
        loop {}
    }

    unsafe fn putc(byte: u8) {
        let _ = sbi::debug_console_write_byte(byte);
    }
}
