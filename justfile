# cargo's build mode, should be "debug" or "release"
mode := "debug"

# QEMU binary to be used
qemu := "qemu-system-riscv64"

BUILD_FLAG := if mode == "release" { "--release" } else { "" }
    
# Build losgatos
build:
    cargo build {{ BUILD_FLAG }}

QEMU_MACHINE_ARGS := '-M virt -serial mon:stdio -kernel -nographic -smp 2'
QEMU_IMAGE := '-kernel target/riscv64gc-unknown-none-elf/' + mode + '/kernel'

# Run losgatos in QEMU
qemu *additional_args: build
    {{ qemu }} {{ QEMU_MACHINE_ARGS }} {{ QEMU_IMAGE }} {{ additional_args }}

# Dump QEMU's device tree to standard output
dump_devicetree:
    {{ qemu }} {{ QEMU_MACHINE_ARGS }} -machine dumpdtb=dtb.dtb 
    dtc -I dtb -O dts dtb.dtb
    rm dtb.dtb

alias b := build
alias d := dump_devicetree
alias q := qemu
