use mmu;

pub const KERNBASE: u32 = 0xF0000000;
pub const KSTKSIZE: u32 = (8 * mmu::PGSIZE); // size of a kernel stack
