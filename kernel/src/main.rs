#![no_std]
#![no_main]

mod csr;
mod debug;
mod entry;
mod sbi;
mod traps;

use core::panic::PanicInfo;
use core::{fmt::Write, ptr::addr_of};
use csr::Csr;
use debug::DebugOutput;
use traps::{wfi, InterruptCode, Traps};

struct Supervisor {
    debug_output: DebugOutput,
}

impl Supervisor {
    pub fn new() -> Supervisor {
        Supervisor {
            debug_output: DebugOutput::new(),
        }
    }

    pub fn launch(&self) -> ! {
        writeln!(&self.debug_output, "Hello, kernel!").unwrap();
        let mut traps = unsafe { Traps::initialize() };
        traps.enable();
        traps.enable_interrupts(InterruptCode::Timer);
        unsafe {
            sbi::timer::set(10000);
        }
        wfi()
    }

    pub unsafe fn set_global(&self) {
        let ptr_addr = addr_of!(*self) as usize;
        unsafe {
            csr::sscratch::write(ptr_addr);
        }
    }

    pub fn global() -> &'static Self {
        // SAFETY:
        // sstatus is initialized in assembly code to 0 - in this case we panic
        // otherwise, it is assumed that it is set by `set_global` function, to a Supervisor value
        let supervisor_ptr = unsafe { csr::sscratch::read() } as *const Supervisor;

        unsafe { supervisor_ptr.as_ref() }.expect("uninitialized global state")
    }

    pub fn debug_output(&self) -> &DebugOutput {
        &self.debug_output
    }
}

#[panic_handler]
fn panic_handler(panic: &PanicInfo) -> ! {
    // do not use global debug output as we're not sure it is correctly initialized
    let debug_output = DebugOutput::new();

    unsafe {
        Traps::initialize().disable();
    }

    writeln!(&debug_output, "Kernel panic: {:?}", panic.message()).unwrap();
    if let Some(loc) = panic.location() {
        writeln!(&debug_output, "at {}", loc).unwrap();
    }
    wfi()
}
