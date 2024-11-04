#![no_std]
#![no_main]

mod csr;
mod debug;
mod entry;
mod sbi;
mod traps;

use core::fmt::Write;
use core::panic::PanicInfo;
use debug::DebugOutput;
use traps::{wfi, Traps};

fn kernel_main() -> ! {
    let dout = DebugOutput::new();
    writeln!(&dout, "Hello kernel!").unwrap();
    wfi()
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
