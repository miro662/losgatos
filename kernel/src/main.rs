#![no_std]
#![no_main]

mod arch;
mod data_structures;
mod debug;
mod memory;
mod sync;

use arch::Arch;
use core::fmt::Write;
use core::panic::PanicInfo;
use debug::DebugOutput;
use memory::map::MemoryMap;

fn kernel_main<A: Arch>(mut memory_map: MemoryMap) -> ! {
    let dout = DebugOutput::<A>::new();
    writeln!(&dout, "xD");
    A::wfi();
}

#[panic_handler]
fn panic_handler(panic: &PanicInfo) -> ! {
    loop {}
}
