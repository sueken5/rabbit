#[inline(always)]
pub unsafe fn inb(port: u16) -> u8 {
    let data: u8;
    asm!("inb %dx, %al" : "={ax}" (data) : "{dx}"(port) :: "volatile");
    return data;
}

/*
The implementation of insl (0412) is worth looking at more closely. Rep insl is
actually a tight loop masquerading as a single instruction. The rep prefix executes the
following instruction %ecx times, decrementing %ecx after each iteration. The insl
instruction reads a 32-bit value from port %dx into memory at address %edi and then
increments %edi by 4. Thus rep insl copies 4×%ecx bytes, in 32-bit chunks, from
port %dx into memory starting at address %edi.
*/

#[inline(always)]
pub unsafe fn insl(port: u16, addr: u32, cnt: u32) {
    asm!("cld; rep insl %dx, (%edi)"
         :
         : "{ecx}"(cnt), "{dx}"(port), "{edi}"(addr)
         : "ecx", "edi", "memory", "cc"
         : "volatile");
}

#[inline(always)]
pub unsafe fn outb(port: u16, data: u8) {
    asm!("outb %al, %dx" :: "{dx}"(port), "{al}"(data) :: "volatile");
}
