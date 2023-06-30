use std::fs::File;
use std::io::{Read, Write};
use common::*;

#[allow(dead_code)]
pub struct Cartridge {
    rom: Vec<u8>,
    ram: Vec<u8>,
    mbc_type: u8,
    ram_enable: bool,
    bank_no_upper: u8,
    bank_no_lower: u8,
    num_rom_banks: u8,
    mode: bool,
    cgb_flg: u8,
}

// https://gbdev.io/pandocs/The_Cartridge_Header.html
impl Cartridge {
    pub fn new(fname: &str) -> Self {
        let mut rom = Vec::new();
        let mut file = File::open(fname).unwrap();
        file.read_to_end(&mut rom).unwrap();

        // CGBフラグ(0x143バイト目)
        let cgb_flg: u8 = rom[0x0143];
        info!("CGB Flag: {:#02X}", cgb_flg);

        let mbc_type = rom[0x0147];
        let mbc_name = match mbc_type {
            0x00 => "ROM ONLY",
            0x01 => "MBC1",
            0x02 => "MBC1+RAM",
            0x03 => "MBC1+RAM+BATTERY",
            0x05 => "MBC2",
            0x06 => "MBC2+BATTERY",
            0x08 => "ROM+RAM",
            0x09 => "ROM+RAM+BATTERY",
            0x0B => "MMM01",
            0x0C => "MMM01+RAM",
            0x0D => "MMM01+RAM+BATTERY",
            0x0F => "MBC3+TIMER+BATTERY",
            0x10 => "MBC3+TIMER+RAM+BATTERY",
            0x11 => "MBC3",
            0x12 => "MBC3+RAM",
            0x13 => "MBC3+RAM+BATTERY",
            0x19 => "MBC5",
            0x1A => "MBC5+RAM",
            0x1B => "MBC5+RAM+BATTERY",
            0x1C => "MBC5+RUMBLE",
            0x1D => "MBC5+RUMBLE+RAM",
            0x1E => "MBC5+RUMBLE+RAM+BATTERY",
            0x20 => "MBC6",
            0x22 => "MBC7+SENSOR+RUMBLE+RAM+BATTERY",
            0xFC => "POCKET CAMERA",
            0xFD => "BANDAI TAMA5",
            0xFE => "HuC3",
            0xFF => "HuC1+RAM+BATTERY",
            _ => "Unknown",
        };

        let rom_size: usize = match rom[0x0148] {
            0 => 32 * 1024,
            n => 32 * 1024 << (n as usize),
        };

        let num_rom_banks = 2 << rom[0x0148];

        let ram_size: usize = match rom[0x0149] {
            0 => 0,
            1 => 2 * 1024,
            2 => 8 * 1024,
            3 => 32 * 1024,
            4 => 128 * 1024,
            5 => 64 * 1024,
            _ => panic!("RAM size invalid"),
        };

        // チェックサム$014D (= $0134~$014C)
        let mut chksum: u8 = 0;
        for i in 0x0134..0x014D {
            chksum = chksum.wrapping_sub(rom[i]).wrapping_sub(1);
        }
        if chksum != rom[0x014D] {
            panic!("ROM header checksum is incorrect");
        }

        if rom_size != rom.len() {
            panic!("ROM file invalid");
        }

        info!("ROM size {}KB", rom_size / 1024);
        info!("RAM size {}KB", ram_size / 1024);
        info!("MBC type {}", mbc_name);

        Cartridge {
            rom: rom,
            ram: vec![0; ram_size],
            mbc_type: mbc_type,
            ram_enable: false,
            bank_no_upper: 0,
            bank_no_lower: 0,
            num_rom_banks: num_rom_banks,
            mode: false,
            cgb_flg: cgb_flg,
        }
    }

    fn rom_bank_no(&self) -> u8 {
        let bank_no = if self.mode {
            self.bank_no_lower
        } else {
            self.bank_no_upper << 5 | self.bank_no_lower
        };

        let bank_no = match bank_no {
            0 | 0x20 | 0x40 | 0x60 => bank_no + 1,
            _ => bank_no,
        };

        bank_no & (self.num_rom_banks - 1)
    }

    fn ram_bank_no(&self) -> u8 {
        if self.mode {
            self.bank_no_upper
        } else {
            0
        }
    }

    pub fn read_save_file(&mut self, fname: &str) {
        info!("Reading save file from: {}", fname);

        if let Ok(mut file) = File::open(fname) {
            self.ram = Vec::new();
            file.read_to_end(&mut self.ram).unwrap();
        }
    }

    pub fn write_save_file(&mut self, fname: &str) {
        info!("Writing save file to: {}", fname);

        if let Ok(mut file) = File::create(fname) {
            file.write_all(&mut self.ram).unwrap();
        }
    }

    pub fn get_cgb_mode(&self) -> u8 {
        self.cgb_flg
    }
}

impl IO for Cartridge {
    fn write(&mut self, addr: u16, val: u8) {
        match addr {
            // RAM enable
            0x0000..=0x1FFF => self.ram_enable = val & 0x0F == 0x0A,
            // ROM bank number (lower 5 bits)
            0x2000..=0x3FFF => self.bank_no_lower = val & 0x1F,
            // RAM bank number or ROM bank number (upper 2 bits)
            0x4000..=0x5FFF => self.bank_no_upper = val & 0x03,
            // ROM/RAM mode select
            0x6000..=0x7FFF => self.mode = val & 0x01 > 0,
            // RAM bank 00-03
            0xA000..=0xBFFF => {
                if !self.ram_enable {
                    return;
                }
                let offset = (8 * 1024) * self.ram_bank_no() as usize;
                self.ram[(addr & 0x1FFF) as usize + offset] = val
            }
            _ => unreachable!("Unexpected address: 0x{:04X}", addr),
        }
    }

    fn read(&mut self, addr: u16) -> u8 {
        match addr {
            // ROM bank 00
            0x0000..=0x3FFF => self.rom[addr as usize],
            // ROM bank 01-7F
            0x4000..=0x7FFF => {
                let offset = (16 * 1024) * self.rom_bank_no() as usize;
                self.rom[(addr & 0x3FFF) as usize + offset]
            }
            // RAM bank 00-03
            0xa000..=0xBFFF => {
                if !self.ram_enable {
                    return 0xFF;
                }
                let offset = (8 * 1024) * self.ram_bank_no() as usize;
                self.ram[(addr & 0x1FFF) as usize + offset]
            }
            _ => unreachable!("Unexpected address: 0x{:04X}", addr),
        }
    }

    fn update(&mut self, _tick: u8) {}
}
