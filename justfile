build:
    cargo build

qemu: build
    qemu-system-riscv64 -M virt -serial mon:stdio -kernel target/riscv64gc-unknown-none-elf/debug/kernel -nographic -smp 2

dumpdt:
    qemu-system-riscv64 -M virt -serial mon:stdio -machine dumpdtb=dtb.dtb -nographic -smp 2
    dtc -I dtb -O dts dtb.dtb
    rm dtb.dtb