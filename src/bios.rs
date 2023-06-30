use std::fs::File;
// use std::io::{Read, Write};
use std::io::Read;
use common::*;

pub struct BIOS {
    pub bios: Vec<u8>,
    pub is_boot: bool
}

impl BIOS {
    pub fn new(path: &str) -> Self {
        let mut bios = Vec::new();
        let mut file = File::open(path).unwrap();
        file.read_to_end(&mut bios).unwrap();

        BIOS {
            bios: bios,
            is_boot: true,
        }
    }
}

#[allow(dead_code)]
impl IO for BIOS {
    fn write(&mut self, addr: u16, _val: u8) {
        match addr {
            _ => panic!("[ERR] BIOS Write Only! (Addr: ${:#04X})", addr),
        }
    }

    fn read(&mut self, addr: u16) -> u8 {
        match addr {
            0x0000..=0x00FF => {
                self.bios[addr as usize]
            },
            _ => panic!("[ERR] BIOS Read Addr ${:#04X}", addr),
        }
    }

    fn update(&mut self, _tick: u8) {
        // NOP
    }
}
