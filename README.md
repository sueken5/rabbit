# 知恵

`volatile` = 揮発性

```
The one-time initialization of statics with non-const functions is a common problem in Rust. Fortunately, there already exists a good solution in a crate named lazy_static. This crate provides a lazy_static! macro that defines a lazily initialized static. Instead of computing its value at compile time, the static laziliy initializes itself when it's accessed the first time. Thus, the initialization happens at runtime so that arbitrarily complex initialization code is possible.
```

`lazy_static` and `spin`, are very useful in OS development

# めも

```
// #![cfg_attr(test, allow(dead_code, unused_macros, unused_imports))]
//
// //![no_std]をつけると標準ライブラリが一切つかえなくなる
// #![no_std]
// //#![no_main]をつけるとRustのランタイムであるcr0を使わなくなる
// #![no_main]
//
// #[cfg(test)]
// extern crate std;
//
// use core::panic::PanicInfo;
// extern crate bootloader_precompiled;
// extern crate volatile;
// extern crate spin;
//
// #[macro_use]
// extern crate lazy_static;
//
// #[macro_use]
// mod vga_buffer;
//
// // The linker just looks for a function with that name and sets this function as entry point the executable.
// #[cfg(not(test))]
// #[no_mangle]
// pub extern "C" fn _start() -> ! {
//     println!("Hello World{}", "!");
//     loop {}
// }
//
// /// This function is called on panic.
// #[cfg(not(test))] // only compile when the test flag is not set. The #[cfg(…)] attribute ensures that the annotated item is only included if the passed condition is met.
// #[panic_handler]
// fn panic(info: &PanicInfo) -> ! {
//     println!("{}", info);
//     loop {}
// }
```
