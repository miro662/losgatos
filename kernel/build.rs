use std::env;

fn main() {
    let sbi_next_address = env::var("SBI_NEXT_ADDRESS").unwrap_or("0x0000000080200000".to_string());

    let watched_files = ["kernel/entry.S", "kernel/linker.ld"];
    for file in watched_files {
        println!("cargo:rerun-if-changed={}", file);
    }

    println!(
        "cargo:rustc-link-arg=-defsym=SBI_NEXT_ADDRESS={}",
        sbi_next_address
    );
    println!("cargo:rustc-link-arg=-Tkernel/src/linker.ld");
}
