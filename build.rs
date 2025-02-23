fn main() {
    println!("cargo:rerun-if-changed=boot.s");
    println!("cargo:rerun-if-changed=linker.ld");

    cc::Build::new()
        .file("boot.s")
        .compile("boot");

    println!("cargo:rustc-link-search=.");
    println!("cargo:rustc-link-arg=-Tlinker.ld");
}
