#![no_std]

#[no_mangle]
pub extern "C" fn main() {
    let vga_buffer = 0xb8000 as *mut u8;

    for (i, &byte) in b"Entry Kernel!".iter().enumerate() {
        unsafe {
            *vga_buffer.offset(i as isize * 2) = byte;
            *vga_buffer.offset(i as isize * 2 + 1) = 0xb;
        }
    }
    loop{}
}
