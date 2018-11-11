#![no_std]
#![no_main]
#![feature(panic_handler)]
#![feature(global_asm)]
#![feature(asm)]

use core::panic::PanicInfo;

global_asm!(include_str!("boot.S"));

#[no_mangle]
pub extern "C" fn bootmain() -> ! {
    loop {}
}

#[panic_handler]
#[no_mangle]
pub extern "C" fn panic(_info: &PanicInfo) -> ! {
    loop {}
}
