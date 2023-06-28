// =========================================================================
// [Common Define]
// =========================================================================
pub const _BIT_0:   u8 = 0x00000001;
pub const _BIT_1:   u8 = 0x00000002;
pub const _BIT_2:   u8 = 0x00000004;
pub const _BIT_3:   u8 = 0x00000008;
pub const _BIT_4:   u8 = 0x00000010;
pub const _BIT_5:   u8 = 0x00000020;
pub const _BIT_6:   u8 = 0x00000040;
pub const _BIT_7:   u8 = 0x00000080;
pub const _BIT_8:  u16 = 0x00000100;
pub const _BIT_9:  u16 = 0x00000200;
pub const _BIT_10: u16 = 0x00000400;
pub const _BIT_11: u16 = 0x00000800;
pub const _BIT_12: u16 = 0x00001000;
pub const _BIT_13: u16 = 0x00002000;
pub const _BIT_14: u16 = 0x00004000;
pub const _BIT_15: u16 = 0x00008000;
pub const _BIT_16: u32 = 0x00010000;
pub const _BIT_17: u32 = 0x00020000;
pub const _BIT_18: u32 = 0x00040000;
pub const _BIT_19: u32 = 0x00080000;
pub const _BIT_20: u32 = 0x00100000;
pub const _BIT_21: u32 = 0x00200000;
pub const _BIT_22: u32 = 0x00400000;
pub const _BIT_23: u32 = 0x00800000;
pub const _BIT_24: u32 = 0x01000000;
pub const _BIT_25: u32 = 0x02000000;
pub const _BIT_26: u32 = 0x04000000;
pub const _BIT_27: u32 = 0x08000000;
pub const _BIT_28: u32 = 0x10000000;
pub const _BIT_29: u32 = 0x20000000;
pub const _BIT_30: u32 = 0x40000000;
pub const _BIT_31: u32 = 0x80000000;

pub const _MEM_SIZE_1K:   u16 =   1 * 1024;
pub const _MEM_SIZE_2K:   u16 =   2 * 1024;
pub const _MEM_SIZE_4K:   u16 =   4 * 1024;
pub const _MEM_SIZE_8K:   u16 =   8 * 1024;
pub const _MEM_SIZE_16K:  u16 =  16 * 1024;
pub const _MEM_SIZE_32K:  u16 =  32 * 1024;
pub const _MEM_SIZE_64K:  u32 =  64 * 1024;
pub const _MEM_SIZE_128K: u32 = 128 * 1024;
pub const _MEM_SIZE_256K: u32 = 256 * 1024;
pub const _MEM_SIZE_512K: u32 = 512 * 1024;

pub const _MMC_0: u8 = 0;
pub const _MMC_1: u8 = 1;
pub const _MMC_2: u8 = 2;
pub const _MMC_3: u8 = 3;
pub const _MMC_4: u8 = 4;

pub const _MAPPER_0: u8 = 0;
pub const _MAPPER_1: u8 = 1;
pub const _MAPPER_2: u8 = 2;
pub const _MAPPER_3: u8 = 3;
pub const _MAPPER_4: u8 = 4;
pub const _MAPPER_105: u8 = 105;
pub const _MAPPER_115: u8 = 115;
pub const _MAPPER_118: u8 = 118;
pub const _MAPPER_119: u8 = 119;

pub const _CHR_ROM: u8 = 0;
pub const _CHR_RAM: u8 = 1;
pub const _PRG_ROM: u8 = 2;

// =========================================================================
pub trait IODevice {
    fn write(&mut self, addr: u16, val: u8);
    fn read(&self, addr: u16) -> u8;
    fn update(&mut self, tick: u8);
}
