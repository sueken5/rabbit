pub const DATA_PORT: u16 = 0x1F0;
pub const SECTOR_COUNT: u16 = 0x1F2;
pub const LOGICAL_BLOCK_ADDRESS_LOW_BYTE: u16 = 0x1F3;
pub const LOGICAL_BLOCK_ADDRESS_MID_BYTE: u16 = 0x1F4;
pub const LOGICAL_BLOCK_ADDRESS_HIGH_BYTE: u16 = 0x1F5;
pub const LOGICAL_BLOCK_ADDRESS_LAST_BYTE: u16 = 0x1F6;
pub const CMD: u16 = 0x1F7;
pub const READ_SECTOR: u8 = 0x20; //cmd 0x20 - read sectors
