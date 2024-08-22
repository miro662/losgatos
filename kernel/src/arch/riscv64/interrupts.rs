use core::{arch::asm, mem::transmute};

pub fn wfi() -> ! {
    // safety: this instruction hangs processor until an interrupt is received
    unsafe { transmute(asm!("wfi")) }
}
