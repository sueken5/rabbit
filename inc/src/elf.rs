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
