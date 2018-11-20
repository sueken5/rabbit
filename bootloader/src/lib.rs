#![no_std]
#![feature(global_asm)]
#![feature(asm)]

use core::panic::PanicInfo;
extern crate inc;
extern crate hwio;
use inc::elf;
use inc::x86;
use hwio::disk;

global_asm!(include_str!("boot.S"));

const SECTOR_SIZE: u32 = 512;
const KB4: u32 = SECTOR_SIZE * 8;

#[no_mangle]
pub unsafe extern "C" fn bootmain() {
    let vga_buffer = 0xb8000 as *mut u8;

    for (i, &byte) in b"Entry BootLoader!".iter().enumerate() {
        *vga_buffer.offset(i as isize * 2) = byte;
        *vga_buffer.offset(i as isize * 2 + 1) = 0xb;
    }

    let elf_header = 0x10000 as *const elf::ELFHeader;

    read_segment(elf_header as u32, KB4, 0);

    // is this a valid ELF?
	if (*elf_header).magic != elf::ELF_MAGIC {
        for (i, &byte) in b"invalid elf magic".iter().enumerate() {
            *vga_buffer.offset(i as isize * 2) = byte;
            *vga_buffer.offset(i as isize * 2 + 1) = 0xb;
        }
        return;
    }

    let mut ph = ((elf_header as *const u8).offset((*elf_header).program_header_table_offset as isize)) as *const elf::ProgramHeader;
    let end_ph = ph.offset((*elf_header).program_header_num as isize);

    while ph < end_ph {
		read_segment((*ph).paddr, (*ph).memsize, (*ph).p_offset);
        ph = ((ph as u32) + 32) as *const elf::ProgramHeader;
    }

	// call the entry point from the ELF header
	// note: does not return!
    let entry: extern "C" fn() -> ! = core::mem::transmute((*elf_header).entry);
    entry();
}

#[inline(never)]
fn read_segment(pa: u32, count: u32, offset: u32) {
    let end_pa = pa + count;
    // round down to sector boundary
    let mut bpa = pa & !(SECTOR_SIZE - 1);

    // translate from bytes to sectors, and kernel starts at sector 1
    let mut sector_offset = (offset / SECTOR_SIZE) + 1;

    // If this is too slow, we could read lots of sectors at a time.
	// We'd write more to memory than asked, but it doesn't matter --
	// we load in increasing order.
    while bpa < end_pa {
       // Since we haven't enabled paging yet and we're using
       // an identity segment mapping (see boot.S), we can
       // use physical addresses directly.  This won't be the
       // case once JOS enables the MMU.
       read_sector(bpa , sector_offset);
       sector_offset += 1;
       bpa += SECTOR_SIZE;
   }
}

fn wait_disk() {
    unsafe {
        // wait for disk ready
        while (x86::inb(disk::CMD) & 0xC0) != 0x40 {};
    }
}

fn read_sector(dst: u32, offset: u32) {
    unsafe {
        // wait for disk to be ready
        wait_disk();
        x86::outb(disk::SECTOR_COUNT, 1); // count = 1
        x86::outb(disk::LOGICAL_BLOCK_ADDRESS_LOW_BYTE, offset as u8);
        x86::outb(disk::LOGICAL_BLOCK_ADDRESS_MID_BYTE, (offset >> 8) as u8);
        x86::outb(disk::LOGICAL_BLOCK_ADDRESS_HIGH_BYTE, (offset >> 16) as u8);
        x86::outb(disk::LOGICAL_BLOCK_ADDRESS_LAST_BYTE, ((offset >> 24) | 0xE0) as u8);
        x86::outb(disk::CMD, disk::READ_SECTOR);
        // wait for disk to be ready
        wait_disk();
        // read a sector
        x86::insl(disk::DATA_PORT, dst, SECTOR_SIZE / 4);
    }
}


#[panic_handler]
#[no_mangle]
pub extern "C" fn panic(_info: &PanicInfo) -> ! {
    loop {}
}
