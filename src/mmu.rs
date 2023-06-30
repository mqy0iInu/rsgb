use common::*;
use bios::BIOS;
use cgb::*;
use cartridge::Cartridge;
use serial::Serial;
use gamepad::GamePad;
use timer::Timer;
use ppu::PPU;

// WRAM(Work RAM)
// const WRAM_SIZE: u16 = 8 * 1024;    // DMG
const WRAM_SIZE: u16 = 32 * 1024;   // CGB (4KB * 8バンク)
const WRAM_BANK_SIZE: u16 = 4 * 1024;
// HRAM(High RAM)
const HRAM_SIZE: u16 = 0x7F;

pub struct MMU {
    pub bios: BIOS,
    pub cartridge: Cartridge,
    pub cgb: CGB,
    wram: [u8; WRAM_SIZE as usize],
    hram: [u8; HRAM_SIZE as usize],
    pub serial: Serial,
    pub gamepad: GamePad,
    timer: Timer,
    pub ppu: PPU,
    pub int_flag: u8,
    pub int_enable: u8,
}

impl MMU {
    pub fn new(bios_path: &str, rom_path: &str) -> Self {
        MMU {
            bios: BIOS::new(bios_path),
            cartridge: Cartridge::new(rom_path),
            cgb: CGB::new(),
            wram: [0; WRAM_SIZE as usize],
            hram: [0; HRAM_SIZE as usize],
            serial: Serial::new(),
            gamepad: GamePad::new(),
            ppu: PPU::new(),
            timer: Timer::new(),
            int_flag: 0,
            int_enable: 0,
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
            0xC000..=0xDFFF => {
                if self.cgb.unlock_flg != false {
                    let offset = WRAM_BANK_SIZE as usize * self.cgb.svbk as usize;
                    self.wram[(addr & 0x1FFF) as usize + offset] = val;
                }else{
                    self.wram[(addr & 0x1FFF) as usize] = val;
                }
            },
            // Echo RAM
            0xE000..=0xFDFF => {
                if self.cgb.unlock_flg != false {
                    let offset: usize = WRAM_BANK_SIZE as usize * self.cgb.svbk as usize;
                    self.wram[((addr - WRAM_SIZE) & 0x1FFF) as usize + offset] = val;
                }else{
                    self.wram[((addr - WRAM_SIZE) & 0x1FFF) as usize] = val;
                }
            },
            // OAM
            0xFE00..=0xFE9F => self.ppu.write(addr, val),
            // GamePad
            0xFF00 => self.gamepad.write(addr, val),
            // Serial
            0xFF01..=0xFF02 => self.serial.write(addr, val),
            // Timer
            0xFF04..=0xFF07 => self.timer.write(addr, val),
            // Interrupt Flag
            0xFF0F => self.int_flag = val,
            // TODO APU
            0xFF10..=0xFF26 => { warn!("APU I/O Write ${:#04X}", addr) },
            // TODO Wave Pattern (APU)
            0xFF30..=0xFF3F => { warn!("Wave Patter I/O Write ${:#04X}", addr); },
            // PPU
            0xFF40..=0xFF45 | 0xFF47..=0xFF4B => self.ppu.write(addr, val),
            // OAM DMA
            0xFF46 => self.oam_dma_start(val),
            // (CGB Only) I/O Reg
            0xFF4D..=0xFF77 => {
                self.cgb.write(addr, val);
                // HDMA5 ($FF55)への書き込みはDMA転送開始
                if addr == 0xFF55 {
                    self.cgb_dma_start();
                }
            },
            // HRAM
            0xFF80..=0xFFFE => self.hram[(addr & HRAM_SIZE) as usize] = val,
            // Interrupt Enable
            0xFFFF => self.int_enable = val,
            _ => panic!("[ERR] Invalid Write Addr ${:#04X}", addr),
        }
    }

    pub fn read(&mut self, addr: u16) -> u8 {
        match addr {
            // BIOS or ROM
            0x0000..=0x7FFF => {
                if self.bios.is_boot != false {
                    if addr > 0x00FF {
                        self.bios.is_boot = false;
                        self.cartridge.read(addr)
                    }else{
                        self.bios.read(addr)
                    }
                }else{
                    self.cartridge.read(addr)
                }
            },
            // VRAM
            0x8000..=0x9FFF => self.ppu.read(addr),
            // External RAM
            0xA000..=0xBFFF => self.cartridge.read(addr),
            // WRAM
            0xC000..=0xDFFF => {
                if self.cgb.unlock_flg != false {
                    let offset = WRAM_BANK_SIZE as usize * self.cgb.svbk as usize;
                    self.wram[(addr & 0x1FFF) as usize + offset]
                }else{
                    self.wram[(addr & 0x1FFF) as usize]
                }
            },
            // Echo RAM
            0xE000..=0xFDFF => {
                if self.cgb.unlock_flg != false {
                    let offset: usize = WRAM_BANK_SIZE as usize * self.cgb.svbk as usize;
                    self.wram[((addr - WRAM_SIZE) & 0x1FFF) as usize + offset]
                }else{
                    self.wram[((addr - WRAM_SIZE) & 0x1FFF) as usize]
                }
            },
            // OAM
            0xFE00..=0xFE9F => self.ppu.read(addr),
            // GamePad
            0xFF00 => self.gamepad.read(addr),
            // Serial
            0xFF01..=0xFF02 => self.serial.read(addr),
            // Timer
            0xFF04..=0xFF07 => self.timer.read(addr),
            // Interrupt flag
            0xFF0F => self.int_flag,
            // TODO APU
            0xFF10..=0xFF26 => { warn!("APU I/O Read ${:#04X}", addr);
                                0xFF },
            // TODO Wave Pattern (APU)
            0xFF30..=0xFF3F => { warn!("Wave Patter I/O Read ${:#04X}", addr);
                                0xFF },
            // PPU
            0xFF40..=0xFF45 | 0xFF47..=0xFF4B => self.ppu.read(addr),
            // (CGB Only) I/O Reg
            0xFF4D..=0xFF77 => self.cgb.read(addr),
            // HRAM
            0xFF80..=0xFFFE => self.hram[(addr & HRAM_SIZE) as usize],
            // Interrupt enable
            0xFFFF => self.int_enable,
            _ => panic!("[ERR] Invalid Read Addr ${:#04X}", addr),
        }
    }

    pub fn update(&mut self, tick: u8) {
        self.ppu.cgb_mode = self.cgb.cgb_mode;
        self.ppu.cgb_unlock_flg = self.cgb.unlock_flg;
        self.ppu.vram_bank = self.cgb.vbk;

        self.bios.update(tick);
        self.cgb.update(tick);
        self.cartridge.update(tick);
        self.ppu.update(tick);
        self.timer.update(tick);
        self.gamepad.update(tick);
        self.serial.update(tick);

        // IRQのポーリング
        self.irq_poll();
    }

    fn irq_poll(&mut self) {
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

    fn cgb_dma_start(&mut self) {
        // TODO CGBの汎用DMA、H-Blank DMAの転送タイミング
        let src_addr: u16 = ((self.cgb.hdma1 as u16) << 8) | self.cgb.hdma2 as u16;
        let dst_addr: u16 = ((self.cgb.hdma3 as u16) << 8) | self.cgb.hdma4 as u16;
        let mut _dma_len: u16 = 0;

        if( self.cgb.hdma5 & _BIT_7) != _CGB_GP_DMA {
            // H-Blank DMAは0x10(16)Byte転送
            _dma_len = 0x10;
        }else{
            // 汎用DMAは0x10~0x800(16~2048)Byte転送
            _dma_len = self.cgb.get_dma_len();
        }

        for i in 0.._dma_len {
            let tmp = self.read(src_addr | i);
            self.write(dst_addr | i, tmp);
        }

        // 転送完了はレジスタを0xFFにする
        self.cgb.hdma5 = 0xFF;
    }

    fn oam_dma_start(&mut self, val: u8) {
        // TODO OAM DMA転送バグの実装
        // https://gbdev.io/pandocs/OAM_Corruption_Bug.html
        let src_base = (val as u16) << 8;
        let dst_base = 0xFE00;

        if val < 0x80 || 0xDF < val {
            panic!("[ERR] Invalid DMA Src Addr ${:#04X}", src_base);
        }

        for i in 0..0xA0 {
            let tmp = self.read(src_base | i);
            self.write(dst_base | i, tmp);
        }
    }
}
