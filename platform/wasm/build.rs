fn main() {
    // 2147000000 bytes = 2 GiB
    println!("cargo:rustc-link-arg=-zstack-size=2147000000")
}
