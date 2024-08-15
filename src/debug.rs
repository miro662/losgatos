/// Debug facilities
use core::fmt;

use crate::{arch::putc, sync::AtomicMutex};

/// Wrapper over sbi::DebugConsole implementing fmt::Write
pub struct DebugOutput;

static DEBUG_MUTEX: AtomicMutex<()> = AtomicMutex::new(());

impl fmt::Write for DebugOutput {
    fn write_str(&mut self, s: &str) -> fmt::Result {
        let _ = DEBUG_MUTEX.lock();
        for byte in s.bytes() {
            unsafe {
                let _ = putc(byte);
            }
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
            use crate::debug::DebugOutput;
            use core::fmt::Write;
            let mut debug_output = DebugOutput;
            write!(&mut debug_output, "{}\n", format_args!($($arg)*)).unwrap();
        }
    }
}
pub(crate) use kdebug;
