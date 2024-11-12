#![no_std]
#![no_main]

mod csr;
mod debug;
mod entry;
mod memory;
mod sbi;
mod traps;

use core::panic::PanicInfo;
use core::{fmt::Write, ptr::addr_of};
use csr::Csr;
use debug::DebugOutput;
use devicetree::{FdtHeader, FlattenedDeviceTree};
use memory::map::MemoryMap;
use traps::{
    disable_interrupts, enable_interrupts, initialize_interrupts, wfi, InterruptCode, InterruptMask,
};

struct Supervisor {
    debug_output: DebugOutput,
}

impl Supervisor {
    pub fn new() -> Supervisor {
        Supervisor {
            debug_output: DebugOutput::new(),
        }
    }

    pub fn launch(&self, devicetree_ptr: *const FdtHeader) -> ! {
        kdebug!("Hello, kernel!");

        unsafe {
            initialize_interrupts();
            enable_interrupts();
            let mask: InterruptMask = InterruptCode::Timer.into();
            mask.enable();
        }
        kdebug!("Initialized interrupts");

        kdebug!("Parsing devicetree at 0x{:x}", devicetree_ptr as usize);
        let fdt = unsafe {
            FlattenedDeviceTree::from_ptr(devicetree_ptr).expect("Empty devicetree pointer")
        };

        kdebug!("Building memory map");
        let memory_map = MemoryMap::build_from_devicetree(&fdt);

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
        disable_interrupts();
    }

    writeln!(&debug_output, "Kernel panic: {:?}", panic.message()).unwrap();
    if let Some(loc) = panic.location() {
        writeln!(&debug_output, "at {}", loc).unwrap();
    }
    wfi()
}
