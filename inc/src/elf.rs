pub const ELF_MAGIC: u32 = 0x464C457FU; /* "\x7FELF" in little endian */

#[repr(C)]
pub struct ProgramHeader {
	pub type: u32,
	pub offset: u32,
	pub vaddr: u32,
	pub paddr: u32,
	pub filesize: u32,
	pub memsize: u32,
	pub flags: u32,
	pub align: u32,
}

#[repr(C)]
pub struct ELFHeader {
	pub magic: u32,// must equal ELF_MAGIC
	pub elf: [u8, 12],
	pub type: u16,
	pub machine: u16,
	pub version: u32,
	pub entry: u32,
	pub program_header_table_offset: u32,
	pub section_header_table_offset: u32,
	pub flags: u32,
	pub elf_header_size: u16,
	pub program_header_entry_size: u16,
	pub program_header_num: u16,
	pub section_header_entry_size: u16,
	pub section_header_num: u16,
	pub section_header_string_index: u16,
};
