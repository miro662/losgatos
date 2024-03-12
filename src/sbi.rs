//! Wrapper arround SBI calls

use core::arch::asm;

#[derive(Debug)]
pub struct Error;
pub type Result = core::result::Result<i64, Error>;

macro_rules! sbi_call {
    {$fname:ident, eid: $eid:expr, fid: $fid:expr} => {
        pub unsafe fn $fname() -> Result {
            let mut error: i64;
            let mut value: i64;

            asm! {
                "mv a7, {extension_id}",
                "mv a6, {fid}",
                "ecall",
                "mv {error}, a0",
                "mv {value}, a1",
                extension_id = in(reg) $eid,
                fid = in(reg) $fid,
                error = out(reg) error,
                value = out(reg) value,
            }

            if error < 0 {
                Err(Error)
            } else {
                Ok(value)
            }
        }
    };

    {$fname:ident, eid: $eid:expr, fid: $fid:expr, args: [$arg0:ident : $arg0_type:ident]} => {
        pub unsafe fn $fname($arg0: $arg0_type) -> Result {
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
                Err(Error)
            } else {
                Ok(value)
            }
        }
    }
}

sbi_call! {
    hart_stop, eid: 0x48534D, fid: 0x1
}

sbi_call! {
    debug_console_write_byte, eid: 0x4442434E, fid: 0x2, args: [byte: u8]
}
