use core::arch::global_asm;

use devicetree::FdtHeader;

use super::{sbi, Riscv64};
use crate::{
    arch::riscv64::csr::{self, Csr},
    debug::DebugOutput,
    kernel_main,
};
use core::fmt::Write;

global_asm!(include_str!("entrypoint.S"));

extern "C" {
    pub fn interrupt_handler();
}

#[no_mangle]
pub extern "C" fn kernel_boot(_hart_id: i32, _devicetree_ptr: *const FdtHeader) -> ! {
    unsafe {
        let interrupt_handler_addr = interrupt_handler as *const () as usize;
        csr::stvec::write(interrupt_handler_addr);

        let sie_mask = 1 << 1;
        let new_sstatus = csr::sstatus::read() | sie_mask;
        csr::sstatus::write(new_sstatus);
        csr::sie::write(0x20);

        let _ = sbi::timer::set(400000);
    }
    kernel_main::<Riscv64>()
}

#[no_mangle]
pub extern "C" fn handle_interrupt() {
    let dout = DebugOutput::<Riscv64>::new();
    let cause = unsafe { csr::scause::read() };
    match cause {
        _clock @ 0x8000000000000005 => {
            writeln!(&dout, "Clock interrupt received").unwrap();
            unsafe { sbi::timer::set(0xffffffffffffffff).unwrap() };
        }
        other => writeln!(&dout, "unknown interrupt {:x}", other).unwrap(),
    }
}
