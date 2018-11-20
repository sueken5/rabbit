#![no_main]
#![no_std]
#![feature(global_asm)]
#![feature(asm)]

use core::panic::PanicInfo;
extern crate inc;

// #[link(name = "readline")]
// extern "C" {
//     static mut PGSHIFT: u32;
//     static mut KERNBASE: u32;
//     static mut KSTKSIZE: u32;
// }
//
// PGSHIFT = inc::mmu::PGSHIFT;
// KERNBASE = inc::memlayout::KERNBASE;
// KSTKSIZE = inc::memlayout::KSTKSIZE;

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

pub extern "C" fn reloc (x: u32) -> u32 {
    return x - inc::memlayout::KERNBASE
}

#[panic_handler]
#[no_mangle]
pub extern "C" fn panic(_info: &PanicInfo) -> ! {
    loop {}
}
