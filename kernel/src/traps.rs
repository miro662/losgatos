use core::arch::asm;
use core::fmt::Write;

use crate::{
    csr::{self, Csr},
    sbi, Supervisor,
};

extern "C" {
    pub fn trap_handler();
}

pub struct Traps;

impl Traps {
    const SIE_MASK: usize = 1 << 1;

    pub unsafe fn initialize() -> Traps {
        let trap_handler_addr = trap_handler as *const () as usize;
        csr::stvec::write(trap_handler_addr);
        Traps
    }

    #[inline]
    unsafe fn trap_handler_initialize() -> (Traps, TrapCause) {
        let mut traps = Traps;
        traps.disable();
        let cause = csr::scause::read();
        (traps, TrapCause(cause))
    }

    pub fn enable(&mut self) {
        unsafe {
            csr::sstatus::set_bits(Self::SIE_MASK);
        };
    }

    pub fn disable(&mut self) {
        unsafe {
            csr::sstatus::clear_bits(Self::SIE_MASK);
        };
    }

    pub fn enable_interrupts(&mut self, mask: impl Into<InterruptMask>) {
        unsafe {
            csr::sie::set_bits(mask.into().0);
        }
    }

    pub fn disable_interrupts(&mut self, mask: impl Into<InterruptMask>) {
        unsafe {
            csr::sie::clear_bits(mask.into().0);
        }
    }
}

#[repr(transparent)]
#[derive(Debug, Clone, Copy)]
pub struct TrapCause(usize);

#[derive(Debug, Clone, Copy)]
pub enum TrapCauseDescription {
    Interrupt(InterruptCode),
    Trap(usize),
}

#[derive(Debug, Clone, Copy)]
pub enum InterruptCode {
    Software,
    Timer,
    External,
    CounterOverflow,
    Reseved(usize),
    Platform(usize),
}

impl From<TrapCause> for TrapCauseDescription {
    fn from(value: TrapCause) -> Self {
        let interrupt_mask = 0x1 << 63;
        let is_interrupt = (value.0 & interrupt_mask) != 0;

        if is_interrupt {
            let code = match value.0 & !interrupt_mask {
                1 => InterruptCode::Software,
                5 => InterruptCode::Timer,
                9 => InterruptCode::External,
                13 => InterruptCode::CounterOverflow,
                external @ (16..) => InterruptCode::Platform(external),
                reserved => InterruptCode::Reseved(reserved),
            };
            TrapCauseDescription::Interrupt(code)
        } else {
            TrapCauseDescription::Trap(value.0)
        }
    }
}

#[repr(transparent)]
#[derive(Debug, Clone, Copy)]
pub struct InterruptMask(usize);

impl From<InterruptCode> for InterruptMask {
    fn from(value: InterruptCode) -> Self {
        let mask = 1
            << match value {
                InterruptCode::Software => 1,
                InterruptCode::Timer => 5,
                InterruptCode::External => 9,
                InterruptCode::CounterOverflow => 13,
                InterruptCode::Platform(c) | InterruptCode::Reseved(c) => c,
            };
        InterruptMask(mask)
    }
}

#[no_mangle]
pub extern "C" fn trap_handler_rs(cause: TrapCause) {
    let cause: TrapCauseDescription = cause.into();
    let supervisor = Supervisor::global();

    match cause {
        TrapCauseDescription::Interrupt(InterruptCode::Timer) => {
            let mut debug_output = supervisor.debug_output();
            writeln!(debug_output, "timer interrupt received").unwrap();
            unsafe { sbi::timer::set(0xffffffffffffffff) }.unwrap();
        }
        other => {
            panic!("unhandled interrupt: {:?}", other)
        }
    }
}

pub fn wfi() -> ! {
    // safety: this instruction hangs processor until an interrupt is received
    unsafe { asm!("wfi") }
    #[allow(clippy::empty_loop)]
    loop {}
}
