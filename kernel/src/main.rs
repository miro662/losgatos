#![no_std]
#![no_main]

mod arch;
mod debug;

use arch::Arch;
use core::fmt::Write;
use core::panic::PanicInfo;
use debug::DebugOutput;

fn kernel_main<A: Arch>() -> ! {
    let dout = DebugOutput::<A>::new();
    writeln!(&dout, "Hello kernel!").unwrap();
    A::wfi();
}

#[panic_handler]
fn panic_handler(_panic: &PanicInfo) -> ! {
    loop {}
}
