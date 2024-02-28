/// Debug facilities

use core::fmt;

use crate::sbi;

/// Wrapper over sbi::DebugConsole implementing fmt::Write
pub struct DebugOutput(pub sbi::DebugConsole);

impl fmt::Write for DebugOutput {
    fn write_str(&mut self, s: &str) -> fmt::Result {
        for byte in s.bytes() {
            self.0.write_byte(byte);
        }
        Ok(())
    }
}

/// Writes a formatted string onto a SBI debug console (if available).
/// 
/// Accepts the same arguments as std::fmt::println!
macro_rules! kdebug {
    ($($arg:tt)*) => {
        {
            if let Some(debug_console) = sbi::DebugConsole::get_if_available() {
                use crate::debug::DebugOutput;
                use core::fmt::Write;
                let mut debug_output = DebugOutput(debug_console);
                write!(&mut debug_output, "{}\n", format_args!($($arg)*)).unwrap();
            }
        }
    }
}
pub(crate) use kdebug;
