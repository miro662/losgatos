mod entry;
mod sbi;

pub unsafe fn putc(ch: u8) {
    unsafe {
        let _ = sbi::debug_console_write_byte(ch);
    }
}
