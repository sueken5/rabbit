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

```
// use core::fmt;
// use spin::Mutex;
// use volatile::Volatile;
//
// lazy_static! {
//     pub static ref WRITER: Mutex<Writer> = Mutex::new(Writer {
//         column_position: 0,
//         color_code: ColorCode::new(Color::Yellow, Color::Black),
//         buffer: unsafe { &mut *(0xb8000 as *mut Buffer) },
//     });
// }
//
// // #[allow(dead_code)] attribute we disable these warnings for the Color enum.
// // #[repr(u8)] attribute each enum variant is stored as an u8
// #[allow(dead_code)]
// #[derive(Debug, Clone, Copy, PartialEq, Eq)]
// #[repr(u8)]
// pub enum Color {
//     Black = 0,
//     Blue = 1,
//     Green = 2,
//     Cyan = 3,
//     Red = 4,
//     Magenta = 5,
//     Brown = 6,
//     LightGray = 7,
//     DarkGray = 8,
//     LightBlue = 9,
//     LightGreen = 10,
//     LightCyan = 11,
//     LightRed = 12,
//     Pink = 13,
//     Yellow = 14,
//     White = 15,
// }
//
// #[derive(Debug, Clone, Copy, PartialEq, Eq)]
// struct ColorCode(u8);
//
// //4bitづつ入れなきゃならないがu4がないので以下の様にしてビットを調整する
// impl ColorCode {
//     fn new(foreground: Color, background: Color) -> ColorCode {
//         ColorCode((background as u8) << 4 | (foreground as u8))
//     }
// }
//
// #[derive(Debug, Clone, Copy, PartialEq, Eq)]
// #[repr(C)] //<= bitの並びが定義順になることを保証する
// struct ScreenChar {
//     ascii_character: u8,
//     color_code: ColorCode,
// }
//
// const BUFFER_HEIGHT: usize = 25;
// const BUFFER_WIDTH: usize = 80;
//
// struct Buffer {
//     chars: [[Volatile<ScreenChar>; BUFFER_WIDTH]; BUFFER_HEIGHT],
// }
//
// pub struct Writer {
//     column_position: usize,
//     color_code: ColorCode,
//     buffer: &'static mut Buffer,
// }
//
// impl Writer {
//     pub fn write_string(&mut self, s: &str) {
//         for byte in s.bytes() {
//             match byte {
//                 // printable ASCII byte or newline
//                 0x20...0x7e | b'\n' => self.write_byte(byte),
//                 // not part of printable ASCII range
//                 _ => self.write_byte(0xfe),
//             }
//
//         }
//     }
//     pub fn write_byte(&mut self, byte: u8) {
//         match byte {
//             b'\n' => self.new_line(),
//             byte => {
//                 if self.column_position >= BUFFER_WIDTH {
//                     self.new_line();
//                 }
//
//                 let row = BUFFER_HEIGHT - 1;
//                 let col = self.column_position;
//
//                 let color_code = self.color_code;
//                 self.buffer.chars[row][col].write(ScreenChar {
//                     ascii_character: byte,
//                     color_code: color_code,
//                 });
//                 self.column_position += 1;
//             }
//         }
//     }
//
//     fn new_line(&mut self) {
//         for row in 1..BUFFER_HEIGHT {
//             for col in 0..BUFFER_WIDTH {
//                 let character = self.buffer.chars[row][col].read();
//                 self.buffer.chars[row - 1][col].write(character);
//             }
//         }
//         self.clear_row(BUFFER_HEIGHT - 1);
//         self.column_position = 0;
//     }
//
//     fn clear_row(&mut self, row: usize) {
//         let blank = ScreenChar {
//             ascii_character: b' ',
//             color_code: self.color_code,
//         };
//         for col in 0..BUFFER_WIDTH {
//             self.buffer.chars[row][col].write(blank);
//         }
//     }
// }
//
// impl fmt::Write for Writer {
//     fn write_str(&mut self, s: &str) -> fmt::Result {
//         self.write_string(s);
//         Ok(())
//     }
// }
//
// macro_rules! print {
//     ($($arg:tt)*) => ($crate::vga_buffer::print(format_args!($($arg)*)));
// }
//
// macro_rules! println {
//     () => (print!("\n"));
//     ($fmt:expr) => (print!(concat!($fmt, "\n")));
//     ($fmt:expr, $($arg:tt)*) => (print!(concat!($fmt, "\n"), $($arg)*));
// }
//
// pub fn print(args: fmt::Arguments) {
//     use core::fmt::Write;
//     WRITER.lock().write_fmt(args).unwrap();
// }
//
// #[cfg(test)]
// mod test {
//     use super::*;
//
//     #[test]
//     fn foo() {}
// }
```
