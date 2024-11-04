//! Interaction with Zicsr instructions

#![allow(non_camel_case_types)]
use core::arch::asm;

/// A RISC-V control and status register
pub unsafe trait Csr {
    unsafe fn write(value: usize);
    unsafe fn read() -> usize;
    unsafe fn set_bits(mask: usize);
    unsafe fn clear_bits(mask: usize);
}

macro_rules! csr {
    ($name:ident) => {
        pub struct $name;

        unsafe impl Csr for $name {
            unsafe fn write(value: usize) {
               asm!(
                concat!("csrw ", stringify!($name), ", {i}"),
                i = in(reg) value
               );
            }

            unsafe fn set_bits(mask: usize) {
               asm!(
                concat!("csrs ", stringify!($name), ", {i}"),
                i = in(reg) mask
               );
            }

            unsafe fn clear_bits(mask: usize) {
               asm!(
                concat!("csrc ", stringify!($name), ", {i}"),
                i = in(reg) mask
               );
            }


            unsafe fn read() -> usize {
               let result: usize;
               asm!(
                "mv {o}, zero",
                concat!("csrr {o}, ", stringify!($name)),
                o = out(reg) result
               );
               result
            }
        }
    };
}

csr!(sstatus);
csr!(stvec);
csr!(sip);
csr!(sie);
csr!(scause);
