.section .text.boot

/// Sets processor in RISC-V ABI compliant state and starts Rust kernel code
.global entrypoint
entrypoint:
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