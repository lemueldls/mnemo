fn main() {
    // 4194000 bytes = 4 MiB
    println!("cargo:rustc-link-arg=-zstack-size=4194000")
}
