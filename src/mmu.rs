use cgb::CGB;
use cartridge::Cartridge;
use common::IO;
use gamepad::GamePad;
use ppu::PPU;
use timer::Timer;

// WRAM(Work RAM)
const WRAM_SIZE: u16 = 8 * 1024;
// HRAM(High RAM)
const HRAM_SIZE: u16 = 0x7F;

pub struct MMU {
    pub cgb: CGB,
    pub cartridge: Cartridge,
    wram: [u8; WRAM_SIZE as usize],
    hram: [u8; HRAM_SIZE as usize],
    pub gamepad: GamePad,
    timer: Timer,
    pub ppu: PPU,
    pub int_flag: u8,
    pub int_enable: u8,
}

impl MMU {
    pub fn new(rom_name: &str) -> Self {
        MMU {
            cartridge: Cartridge::new(rom_name),
            cgb: CGB::new(),
            wram: [0; WRAM_SIZE as usize],
            hram: [0; HRAM_SIZE as usize],
            gamepad: GamePad::new(),
            ppu: PPU::new(),
            timer: Timer::new(),
            int_flag: 0,
            int_enable: 0,
        }
    }

    // TODO OAM DMA Timing
    fn do_dma(&mut self, val: u8) {
        if val < 0x80 || 0xdf < val {
            panic!("Invalid DMA source address")
        }

        let src_base = (val as u16) << 8;
        let dst_base = 0xfe00;

        for i in 0..0xA0 {
            let tmp = self.read(src_base | i);
            self.write(dst_base | i, tmp);
        }
    }

    pub fn write(&mut self, addr: u16, val: u8) {
        match addr {
            // ROM
            0x0000..=0x7FFF => self.cartridge.write(addr, val),
            // VRAM
            0x8000..=0x9FFF => self.ppu.write(addr, val),
            // External RAM
            0xA000..=0xBFFF => self.cartridge.write(addr, val),
            // WRAM
            0xC000..=0xDFFF => self.wram[(addr & 0x1FFF) as usize] = val,
            // WRAM Mirror
            0xE000..=0xFDFF => self.wram[((addr - WRAM_SIZE) & 0x1FFF) as usize] = val,
            // OAM
            0xFE00..=0xFE9F => self.ppu.write(addr, val),
            // GamePad
            0xFF00 => self.gamepad.write(addr, val),
            // Timer
            0xFF04..=0xFF07 => self.timer.write(addr, val),
            // Interrupt flag
            0xFF0F => self.int_flag = val,
            // PPU
            0xFF40..=0xFF45 | 0xFF47..=0xFF4B => self.ppu.write(addr, val),
            // OAM DMA
            0xFF46 => self.do_dma(val),
            // HRAM
            0xFF80..=0xFFFE => self.hram[(addr & HRAM_SIZE) as usize] = val,
            // Interrupt enable
            0xFFFF => self.int_enable = val,
            _ => (),
        }
    }

    /// Reads a byte from an address.
    pub fn read(&self, addr: u16) -> u8 {
        match addr {
            // ROM
            0x0000..=0x7FFF => self.cartridge.read(addr),
            // VRAM
            0x8000..=0x9FFF => self.ppu.read(addr),
            // External wram
            0xA000..=0xBFFF => self.cartridge.read(addr),
            // wram
            0xC000..=0xDFFF => self.wram[(addr & 0x1FFF) as usize],
            // Echo wram
            0xE000..=0xFDFF => self.wram[((addr - WRAM_SIZE) & 0x1FFF) as usize],
            // OAM
            0xFE00..=0xFE9F => self.ppu.read(addr),
            // GamePad
            0xFF00 => self.gamepad.read(addr),
            // Timer
            0xFF04..=0xFF07 => self.timer.read(addr),
            // Interrupt flag
            0xFF0f => self.int_flag,
            // PPU
            0xFF40..=0xFF45 | 0xFF47..=0xFF4B => self.ppu.read(addr),
            // HRAM
            0xFF80..=0xFFFE => self.hram[(addr & HRAM_SIZE) as usize],
            // Interrupt enable
            0xFFFF => self.int_enable,
            _ => 0xFF,
        }
    }

    /// Progresses the clock for a given number of ticks.
    pub fn update(&mut self, tick: u8) {
        self.cartridge.update(tick);
        self.ppu.update(tick);
        self.timer.update(tick);
        self.gamepad.update(tick);

        if self.ppu.irq_vblank {
            self.int_flag |= 0x01;
            self.ppu.irq_vblank = false;
        }

        if self.ppu.irq_lcdc {
            self.int_flag |= 0x02;
            self.ppu.irq_lcdc = false;
        }

        if self.timer.irq {
            self.int_flag |= 0x04;
            self.timer.irq = false;
        }

        if self.gamepad.irq {
            self.int_flag |= 0x10;
            self.gamepad.irq = false;
        }
    }
}
