use std::process::Command;

fn main() {
    let bootimage_path = env!("OUT_DIR").to_string() + "/bootimage.bin";

    println!("Running QEMU with image: {}", bootimage_path);

    Command::new("qemu-system-x86_64")
        .args([
            "-drive",
            &format!("format=raw,file={}", bootimage_path),
            "-serial",
            "stdio",
        ])
        .spawn()
        .expect("Failed to start QEMU");
}
