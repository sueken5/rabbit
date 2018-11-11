#![no_std]
#![feature(panic_handler)]

use core::panic::PanicInfo;

#[no_mangle]
pub extern "C" fn i386_init() {
    loop {}
}

#[panic_handler]
#[no_mangle]
pub extern "C" fn panic(_info: &PanicInfo) -> ! {
    loop {}
}
