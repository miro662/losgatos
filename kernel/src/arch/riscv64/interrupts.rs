use core::arch::asm;

pub fn wfi() -> ! {
    // safety: this instruction hangs processor until an interrupt is received
    unsafe { asm!("wfi") }
    #[allow(clippy::empty_loop)]
    loop {}
}
