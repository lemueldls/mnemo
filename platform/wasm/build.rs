use std::env;

fn main() {
    let profile = env::var("PROFILE").unwrap_or_default();
    let target_env = env::var("CARGO_CFG_TARGET_ENV").unwrap_or_default();

    println!("cargo:rustc-cfg=wasm_target");

    // Enhanced WASM optimizations
    println!("cargo:rustc-env=WASM_BINDGEN_WEAKREF=1");
    println!("cargo:rustc-env=WASM_BINDGEN_EXTERNREF_XFORM=1");

    // Enable SIMD if supported
    if env::var("CARGO_FEATURE_WASM_SIMD").is_ok() {
        println!("cargo:rustc-cfg=wasm_simd");
        println!("cargo:rustc-target-feature=+simd128");
    }

    // WASM-specific link arguments for size optimization
    if profile == "release" {
        println!("cargo:rustc-link-arg=--no-entry");
        println!("cargo:rustc-link-arg=--gc-sections");
        println!("cargo:rustc-link-arg=--strip-all");
        println!("cargo:rustc-link-arg=-zstack-size=2147000000 "); // 2 GiB

        // Enable bulk memory operations
        println!("cargo:rustc-target-feature=+atomics,+bulk-memory,+mutable-globals");
        // println!("cargo:rustc-link-arg=--enable-bulk-memory");
        // println!("cargo:rustc-link-arg=--enable-mutable-globals");
    }

    // Web-specific features
    if target_env == "unknown" {
        println!("cargo:rustc-cfg=web_target");
    }
}
