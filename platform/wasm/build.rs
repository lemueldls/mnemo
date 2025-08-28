fn main() {
    println!("cargo:rustc-link-arg=-zstack-size=4194000") // 4 MiB
}
