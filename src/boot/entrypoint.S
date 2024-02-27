.section .text.boot

.global entrypoint
entrypoint:
    li a7, 0x4442434E // debug console extension
    li a6, 0x2 // console write byte
    li a0, 'x'
    ecall

    li a7, 0x4442434E // debug console extension
    li a6, 0x2 // console write byte
    li a0, 'D'
    ecall

1:
    j 1b