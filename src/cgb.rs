use common::*;

pub const CGB_MODE_DMG_COMPATI: u8 = 0x80;  // CGB機能とDMGコンパチ動作（GB/GBC共通カートリッジ）
pub const CGB_MODE_CGB: u8 = 0xC0;          // CGBでのみ動作（GBC専用カートリッジ）
pub const CGB_MODE_NON_CGB: u8 = 0xAA;      // 非CGBモード（CGBモノクロ動作、DMGでいいかも）
pub const CGB_MODE_NONE: u8 = 0xFF;
pub const _CGB_GP_DMA: u8 = 0;
pub const _CGB_H_BLANK_DMA: u8 = 1;
pub const COLOR_PALETTE_SIZE: usize = 64;

// [CGB対応]
// TODO :MBC1（GB/GBC共通） ... テリーのワンダーランド
// TODO :MBC3（GB/GBC共通） ... ポケモン（金、銀）
// TODO :MBC3 (GBC専用)     ... ポケモン（クリスタル）
// TODO :MBC5（GB/GBC共通） ... DQ1&2、ゼルダ夢をみる島DX
// TODO :MBC5 (GBC専用)     ... DQ3、マリオDX

// [リファレンス]
// https://gbdev.io/pandocs/CGB_Registers.html
// https://gbdev.io/pandocs/Palettes.html#lcd-color-palettes-cgb-only
// ↓レジスタ一覧
// https://gbdev.io/pandocs/Hardware_Reg_List.html?highlight=bcps#hardware-registers
#[allow(dead_code)]
pub struct CGB {
    pub key_1: u8,          // (Addr $FF4D R/W) スピードスイッチの準備
    pub vbk: u8,            // (Addr $FF4F R/W) VRAM バンク
    pub hdma1: u8,          // (Addr $FF51 W) VRAM DMA ソース（上位）
    pub hdma2: u8,          // (Addr $FF52 W) VRAM DMA ソース（下位）
    pub hdma3: u8,          // (Addr $FF53 W) VRAM DMA 宛先（上位）
    pub hdma4: u8,          // (Addr $FF54 W) VRAM DMA 宛先（下位）
    pub hdma5: u8,          // (Addr $FF55 W) VRAM DMA 長さ/モード/開始
    pub rp: u8,             // (Addr $FF56 R/W) 赤外線通信ポート
    pub bgpi: u8,           // (Addr $FF68 R/W) 背景パレット インデックス
    pub bcps: u8,           // (Addr $FF68 R/W) 背景色パレット仕様
    pub bg_color_palette: Vec<u8>, // (Addr $FF69 R/W) BCPD/BGPD
    pub ocps: u8,           // (Addr $FF6A R/W) OBJ カラーパレット仕様
    pub obpi: u8,           // (Addr $FF6A R/W) OBJ パレットインデックス
    pub obj_color_palette: Vec<u8>, // (Addr $FF6B R/W) OCPD/OBPD
    pub opri: u8,           // (Addr $FF6C R/W) オブジェクト優先モード
    pub svbk: u8,           // (Addr $FF70 R/W) WRAM バンク
    pub pcm12: u8,          // (Addr $FF76 R) Audio digital outputs 1 & 2
    pub pcm34: u8,          // (Addr $FF77 R) Audio digital outputs 3 & 4


    pub unlock_flg: bool,   // アンロックフラグ
    pub cgb_mode: u8,       // CGBモード
}

impl CGB {
    pub fn new() -> Self {
        CGB {
            key_1: 0,
            vbk: 0,
            hdma1: 0,
            hdma2: 0,
            hdma3: 0,
            hdma4: 0,
            hdma5: 0,
            rp: 0,
            bcps: 0,
            bgpi: 0,
            bg_color_palette: vec![0; COLOR_PALETTE_SIZE],
            ocps: 0,
            obpi: 0,
            obj_color_palette: vec![0; COLOR_PALETTE_SIZE],
            opri: 0,
            svbk: 0,
            pcm12: 0,
            pcm34: 0,

            unlock_flg: false,
            cgb_mode: CGB_MODE_NONE,
        }
    }

    pub fn cgb_unlock(&mut self, cgb_flg: u8) {
        // TODO :CGB機能のアンロック
        match cgb_flg {
            0x80 => self.cgb_mode = CGB_MODE_DMG_COMPATI,
            0xC0 => self.cgb_mode = CGB_MODE_CGB,
            _ => {
                self.cgb_mode = CGB_MODE_NON_CGB;
                warn!("[Warn] Old Cartridge??? (CGB Flag: {:#02X})", cgb_flg);
            },
        }
        info!("CGB Flag: {:#02X}", cgb_flg);

        self.unlock_flg = true;
    }

    pub fn get_dma_len(&self) -> u16 {
        let dma_len: u8 = self.hdma5 & 0x7F; // DMA transfer length Bit[6:0]

        // HDMA5のBit[6:0]の0x00~0x1Fを、+1して0x10倍すると0x10~0x800(16~2048)Byteになる
        // 原文(https://gbdev.io/pandocs/CGB_Registers.html#ff55--hdma5-cgb-mode-only-vram-dma-lengthmodestart)
        let transfer_length: u16 = (dma_len as u16 + 0x0001) * 0x0010;
            transfer_length
    }
}

#[allow(dead_code)]
impl IO for CGB {
    fn write(&mut self, addr: u16, val: u8) {
        match addr {
            0xFF4D => self.key_1 = val & 0x01, // Bit7はRO,Bit0はR/W
            0xFF4F => self.vbk = val,
            0xFF51 => self.hdma1 = val,
            0xFF52 => self.hdma2 = val,
            0xFF53 => self.hdma3 = val,
            0xFF54 => self.hdma4 = val,
            0xFF55 => self.hdma5 = val,
            0xFF56 => self.rp = val,
            0xFF68 => {
                // BGPI(Bit7) = インクリメント方法
                self.bgpi = (val & _BIT_7) >> 7;
                // BCPS(Bit[5:0]) = パレットのインデックス
                self.bcps = val & 0x3F;
            },
            0xFF69 => {
                // BCPD/BGPD
                self.bg_color_palette[self.bcps as usize] = val & 0xFF;
                if self.bgpi != 0 {
                    self.bcps = (self.bcps + 1) & 0x3F;
                }
            },
            0xFF6A => {
                // OBPI(Bit7) = インクリメント方法
                self.obpi = (val & _BIT_7) >> 7;
                // OCPS(Bit[5:0]) = パレットのインデックス
                self.ocps = val & 0x3F;
            },
            0xFF6B => {
                // OCPD/OBPD
                self.obj_color_palette[self.ocps as usize] = val & 0xFF;
                if self.obpi != 0 {
                    self.ocps = (self.ocps + 1) & 0x3F;
                }
            },
            0xFF6C => self.opri = val,
            0xFF70 => self.svbk = val,
            _ => panic!("[ERR] CGB Reg Invalid Addr (Write to: ${:#04X})", addr),
        }
    }

    fn read(&mut self, addr: u16) -> u8 {
        match addr {
            0xFF4D => self.key_1,
            0xFF4F => self.vbk,
            0xFF56 => self.rp,
            0xFF68 => {
                // BGPI(Bit7) | BCPS(Bit[5:0])
                self.bgpi << 7 | self.bcps
            },
            0xFF69 => self.bg_color_palette[self.bcps as usize],
            0xFF6A => {
                // OBPI(Bit7) | OCPS(Bit[5:0])
                self.obpi << 7 | self.ocps
            },
            0xFF6B => self.obj_color_palette[self.ocps as usize],
            0xFF6C => self.opri,
            0xFF70 => self.svbk,
            0xFF76 => self.pcm12,
            0xFF77 => self.pcm34,
            _ => panic!("[ERR] CGB Reg Invalid Addr (Read to: ${:#04X})", addr),
        }
    }

    fn update(&mut self, _tick: u8) {
        // TODO
    }
}