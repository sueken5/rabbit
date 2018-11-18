#![no_std]
#![no_main]
#![feature(global_asm)]
#![feature(asm)]

use core::panic::PanicInfo;
extern crate inc;

pub extern "C" {
    use inc::mmu::{PGSHIFT};
    use inc::memlayout::{KERNBASE, KSTKSIZE};
}

global_asm!(include_str!("entry.S"));

#[no_mangle]
pub extern "C" fn kmain() {
    let vga_buffer = 0xb8000 as *mut u8;

    for (i, &byte) in b"Entry Kernel!".iter().enumerate() {
        unsafe {
            *vga_buffer.offset(i as isize * 2) = byte;
            *vga_buffer.offset(i as isize * 2 + 1) = 0xb;
        }
    }
    loop{}
}

#[panic_handler]
#[no_mangle]
pub extern "C" fn panic(_info: &PanicInfo) -> ! {
    loop {}
}
