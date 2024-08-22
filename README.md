# losgatos

a simple operating system (riscv64-only for now)

## Running

### Requirements

* Rust toolchain with `riscv64gc-unknown-none-elf` support. If you use [rustup](https://rustup.rs), you can use `rustup target add riscv64gc-unknown-none-elf` to install one.
* `qemu-system-riscv64`
* [just](https://github.com/casey/just)

### Running losgatos

```bash
$ just qemu
```