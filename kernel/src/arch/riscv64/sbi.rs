//! Wrapper arround SBI calls

#[derive(Debug)]
pub struct Error;
pub type Result = core::result::Result<i64, Error>;

macro_rules! sbi_call {
    {$fname:ident, eid: $eid:expr, fid: $fid:expr, args: [$arg0:ident : $arg0_type:ident]} => {
        pub unsafe fn $fname($arg0: $arg0_type) -> super::Result {
            use core::arch::asm;

            let mut error: i64;
            let mut value: i64;

            asm! {
                "mv a7, {extension_id}",
                "mv a6, {fid}",
                "mv a0, {arg0}",
                "ecall",
                "mv {error}, a0",
                "mv {value}, a1",
                extension_id = in(reg) $eid,
                fid = in(reg) $fid,
                arg0 = in(reg) $arg0,
                error = out(reg) error,
                value = out(reg) value,
            }

            if error < 0 {
                Err(super::Error)
            } else {
                Ok(value)
            }
        }
    }
}

pub mod timer {
    const TIME_EID: usize = 0x54494D45;
    sbi_call! {
        set, eid: TIME_EID, fid: 0x0, args: [stime_value: u64]
    }
}

pub mod debug_console {
    const DEBUG_CONSOLE_EID: usize = 0x4442434E;
    sbi_call! {
        write_byte, eid: DEBUG_CONSOLE_EID, fid: 0x2, args: [byte: u8]
    }
}
