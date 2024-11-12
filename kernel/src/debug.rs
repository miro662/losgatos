use core::fmt;
use core_lib::sync::AtomicMutex;

use crate::sbi;

pub struct DebugOutput {
    mutex: AtomicMutex<()>,
}

impl DebugOutput {
    pub fn new() -> DebugOutput {
        DebugOutput {
            mutex: AtomicMutex::new(()),
        }
    }
}

impl<'a> fmt::Write for &'a DebugOutput {
    fn write_str(&mut self, s: &str) -> fmt::Result {
        let _ = self.mutex.lock();
        for byte in s.as_bytes() {
            unsafe {
                sbi::debug_console::write_byte(*byte).unwrap();
            };
        }
        Ok(())
    }
}

#[macro_export]
macro_rules! kdebug {
    ($($arg:tt)*) => {
        {
            use crate::Supervisor;
            use core::fmt::Write;
            let mut debug_output = Supervisor::global().debug_output();
            writeln!(debug_output, "[{}:{}] {}", file!(), line!(), format_args!($($arg)*)).unwrap()
        }
    };
}
