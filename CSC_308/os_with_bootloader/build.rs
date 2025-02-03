use std::{env, fs, path::PathBuf, process::Command};

fn main() {
    println!("cargo:rerun-if-changed=build.rs");

    let out_dir = PathBuf::from(env::var("OUT_DIR").unwrap());

    // Build the kernel
    let kernel_path = "kernel_with_bootloader";
    let target = concat!(env!("CARGO_MANIFEST_DIR"), "/kernel_with_bootloader/x86_64-bootimage.json");

    let status = Command::new("cargo")
        .args(["build", "--target", target]) // Removed --release to allow both modes
        .current_dir(kernel_path)
        .status()
        .expect("Failed to build kernel");

    if !status.success() {
        panic!("Kernel build failed!");
    }

    // Check for both release and debug binaries
    let kernel_bin_release = PathBuf::from("kernel_with_bootloader/target/x86_64-bootimage/release/kernel_with_bootloader");
    let kernel_bin_debug = PathBuf::from("kernel_with_bootloader/target/x86_64-bootimage/debug/kernel_with_bootloader");

    let kernel_bin = if kernel_bin_release.exists() {
        kernel_bin_release
    } else if kernel_bin_debug.exists() {
        kernel_bin_debug
    } else {
        panic!("Kernel binary not found in debug or release mode!");
    };

    // Copy the selected binary
    let bootloader_bin = out_dir.join("bootimage.bin");
    fs::copy(&kernel_bin, &bootloader_bin).expect("Failed to copy kernel binary");
}
