use core::{fmt, marker::PhantomData};

use crate::{arch::Arch, sync::AtomicMutex};

pub struct DebugOutput<A: Arch> {
    arch: PhantomData<A>,
    mutex: AtomicMutex<()>,
}

impl<A: Arch> DebugOutput<A> {
    pub fn new() -> DebugOutput<A> {
        DebugOutput {
            arch: PhantomData::default(),
            mutex: AtomicMutex::new(()),
        }
    }
}

impl<'a, A: Arch> fmt::Write for &'a DebugOutput<A> {
    fn write_str(&mut self, s: &str) -> fmt::Result {
        let _ = self.mutex.lock();
        for byte in s.as_bytes() {
            unsafe {
                A::putc(*byte);
            }
        }
        Ok(())
    }
}
