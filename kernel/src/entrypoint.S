.section .text.boot

/// Sets processor in RISC-V ABI compliant state and starts Rust kernel code
.global entrypoint
entrypoint:
    // disable interrupts
    csrw sie, zero

    // reset memory mapping
    csrw satp, zero

    // clear bss
    la t1, _bss_start
    la t2, _bss_end
1:
    sd zero, (t1)
    addi t1, t1, 8
    blt t2, t1, 1b

    // initialize stack
    la sp, _stack_end

    // a0 - hart id - is preserved
    tail kernel_boot

.section .text

.global trap_handler
.align 4
trap_handler:
    // store caller-saved registers
    addi sp, sp, -16*8
    sd ra, 0*8(sp)
    sd a0, 1*8(sp)
    sd a1, 2*8(sp)
    sd a2, 3*8(sp)
    sd a3, 4*8(sp)
    sd a4, 5*8(sp)
    sd a5, 6*8(sp)
    sd a6, 7*8(sp)
    sd a7, 8*8(sp)
    sd t0, 9*8(sp)
    sd t1, 10*8(sp)
    sd t2, 11*8(sp)
    sd t3, 12*8(sp)
    sd t4, 13*8(sp)
    sd t5, 14*8(sp)
    sd t6, 15*8(sp)

    call handle_trap

    // restore caller-saved registers
    ld ra, 0*8(sp)
    ld a0, 1*8(sp)
    ld a1, 2*8(sp)
    ld a2, 3*8(sp)
    ld a3, 4*8(sp)
    ld a4, 5*8(sp)
    ld a5, 6*8(sp)
    ld a6, 7*8(sp)
    ld a7, 8*8(sp)
    ld t0, 9*8(sp)
    ld t1, 10*8(sp)
    ld t2, 11*8(sp)
    ld t3, 12*8(sp)
    ld t4, 13*8(sp)
    ld t5, 14*8(sp)
    ld t6, 15*8(sp)
    addi sp, sp, 16*8
    sret