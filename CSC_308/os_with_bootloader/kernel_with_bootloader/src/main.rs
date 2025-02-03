#![no_std]
#![no_main]

use bootloader_api::{entry_point, BootInfo, BootloaderConfig, config::Mapping};
use x86_64::instructions::hlt;

// Use the entry_point macro to register the entry point function
pub static BOOTLOADER_CONFIG: BootloaderConfig = {
    let mut config = BootloaderConfig::new_default();
    config.mappings.physical_memory = Some(Mapping::Dynamic);
    config.kernel_stack_size = 100 * 1024; // 100 KiB
    config
};

// Register the entry point function with the custom config
entry_point!(my_entry_point, config = &BOOTLOADER_CONFIG);

mod writer;
use writer::FrameBufferWriter;

fn my_entry_point(boot_info: &'static mut bootloader_api::BootInfo) -> ! {
    let framebuffer = boot_info.framebuffer.as_mut().unwrap();
    let frame_buffer_info = framebuffer.info();
    let buffer = framebuffer.buffer_mut();

    let mut writer = FrameBufferWriter::new(buffer, frame_buffer_info);

    use core::fmt::Write;
    writeln!(writer, "Hello, World!\nThis is a test.\n").unwrap();

    loop {
        x86_64::instructions::hlt();
    }
}



// Panic handler to prevent Rust from calling std library functions
#[panic_handler]
fn panic(_info: &core::panic::PanicInfo) -> ! {
    loop {
        hlt();
    }
}
