# 知恵

`volatile` = 揮発性

```
The one-time initialization of statics with non-const functions is a common problem in Rust. Fortunately, there already exists a good solution in a crate named lazy_static. This crate provides a lazy_static! macro that defines a lazily initialized static. Instead of computing its value at compile time, the static laziliy initializes itself when it's accessed the first time. Thus, the initialization happens at runtime so that arbitrarily complex initialization code is possible.
```

`lazy_static` and `spin`, are very useful in OS development

# めも

# build

kernel以下をビルド、オブジェクトファイルにしてからブートローダーのオブジェクトファイルと接続させる。

# xargo

`objcopy -O binary -S target/x86_64-bootloader/release/bootloader bootimage.bin`

`qemu-system-x86_64 -hda bootimage.bin -d int -s`

# 資料
- http://qa.elecom.co.jp/faq_detail.html?id=5439
- https://ceunican.github.io/aos/36.IO_Devices.pdf
- http://msyksphinz.hatenablog.com/entry/2016/03/27/060000
- http://softwaretechnique.jp/OS_Development/Tips/IA32_Instructions/STI.html
- http://marycore.jp/prog/c-lang/data-type-ranges-and-bit-byte-sizes/
- http://softwaretechnique.jp/OS_Development/Tips/ELF/elf01.html
- https://doc.rust-jp.rs/the-rust-programming-language-ja/1.6/book/casting-between-types.html
