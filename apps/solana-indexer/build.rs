fn main() {
    // Enable Rust 2024 edition features
    println!("cargo:rustc-check-cfg=cfg(feature, values(\"yellowstone\", \"database\", \"production\", \"development\"))");
}