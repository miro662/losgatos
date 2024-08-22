mod entry;
mod interrupts;
mod sbi;

pub use interrupts::wfi;

pub unsafe fn putc(ch: u8) {
    unsafe {
        let _ = sbi::debug_console_write_byte(ch);
    }
}
