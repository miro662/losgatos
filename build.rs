fn main() {
    let watched_files = ["src/entrypoint.S", "src/linker.ld"];
    for file in watched_files {
        println!("cargo:rerun-if-changed=src/{}", file);
    }

    println!("cargo:rustc-link-arg=-Tsrc/linker.ld");
}
