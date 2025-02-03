#![no_std]
#![no_main] 

use core::panic::PanicInfo;


#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}


const VGA_BUFFER: *mut u8 = 0xb8000 as *mut u8;
const VGA_WIDTH: usize = 80; 
const VGA_HEIGHT: usize = 25;
const TAB_SIZE: usize = 4;


const DEFAULT_COLOR: u8 = 0x0F; // White
const RED_COLOR: u8 = 0x04; // Red
const GREEN_COLOR: u8 = 0x02; // Green
const BLUE_COLOR: u8 = 0x01; // Blue


static mut CURSOR_ROW: usize = 0;
static mut CURSOR_COL: usize = 0;
static mut CURRENT_COLOR: u8 = DEFAULT_COLOR;


unsafe fn write_char(row: usize, col: usize, c: u8, color: u8) {
    let index = (row * VGA_WIDTH + col) * 2; 
    *VGA_BUFFER.add(index) = c; 
    *VGA_BUFFER.add(index + 1) = color; 
}

unsafe fn clear_row(row: usize) {
    for col in 0..VGA_WIDTH {
        write_char(row, col, b' ', DEFAULT_COLOR);
    }
}


unsafe fn scroll_screen() { 
    for row in 1..VGA_HEIGHT {
        for col in 0..VGA_WIDTH { 
            let src_index = (row * VGA_WIDTH + col) * 2; 
            let dst_index = ((row - 1) * VGA_WIDTH + col) * 2;
            *VGA_BUFFER.add(dst_index) = *VGA_BUFFER.add(src_index); 
            *VGA_BUFFER.add(dst_index + 1) = *VGA_BUFFER.add(src_index + 1); 
        }
    }
    clear_row(VGA_HEIGHT - 1); 
    CURSOR_ROW = VGA_HEIGHT - 1;
}


unsafe fn new_line() {
    CURSOR_ROW += 1;
    CURSOR_COL = 0; 
    if CURSOR_ROW >= VGA_HEIGHT {
        scroll_screen();
    }
}

pub fn print(s: &str) {
    unsafe {
        let mut chars = s.bytes();
        while let Some(byte) = chars.next() {
            match byte {
                b'\n' => new_line(), 
                b'\t' => CURSOR_COL = (CURSOR_COL + TAB_SIZE) & !(TAB_SIZE - 1), 
                b'\\' => { 
                    
                    if let Some(b'c') = chars.next() {
                        if let Some(color_code) = chars.next() {
                            CURRENT_COLOR = match color_code {
                                b'r' => RED_COLOR,   // Red
                                b'g' => GREEN_COLOR, // Green 
                                b'b' => BLUE_COLOR,  // Blue
                                b'w' => DEFAULT_COLOR, // Reset to default color (White)
                                _ => CURRENT_COLOR, // Ignore invalid codes
                            };
                            continue;
                        }
                    }
                    write_char(CURSOR_ROW, CURSOR_COL, b'\\', CURRENT_COLOR);
                    CURSOR_COL += 1;
                }
                _ => {
                    write_char(CURSOR_ROW, CURSOR_COL, byte, CURRENT_COLOR);
                    CURSOR_COL += 1;

                    // Move to the next line if we reach the screen width
                    if CURSOR_COL >= VGA_WIDTH {
                        new_line();
                    }
                }
            }
        }
    }
}

/// Starts the program and displays formatted text using the `print` function.
#[no_mangle] // Prevent name mangling for `_start` 
pub extern "C" fn _start() -> ! {


    print("\n");
    print("This my Operating System\n"); 
    print("\n");  
    print("\\crRED TEXT\n"); 
    print("\n");  
    print("\\cgGREEN TEXR\n"); 
    print("\n");  
    print("\\cbBLUE TEXT\n"); 
    print("\n"); 
    print("\\cwRandom line of text\n");
    print("\n");
    print("cwMy name is \t Nosa\n");
    print("\n");
    print("........................\n");
    loop {} // Infinite loop to prevent exiting
}
