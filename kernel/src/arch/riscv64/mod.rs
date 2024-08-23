mod entry;
mod interrupts;
mod sbi;

pub use interrupts::wfi;

pub const PAGE_SIZE: usize = 4096;

pub unsafe fn putc(ch: u8) {
    unsafe {
        let _ = sbi::debug_console_write_byte(ch);
    }
}
