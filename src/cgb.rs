use common::*;

const CGB_MODE_DMG_COMPATI: u8 = 0x80;  // CGB機能とDMGコンパチ動作（GB/GBC共通カートリッジ）
const CGB_MODE_CGB: u8 = 0xC0;          // CGBでのみ動作（GBC専用カートリッジ）
const CGB_MODE_NON_CGB: u8 = 0xAA;      // 非CGBモード（CGBモノクロ動作、DMGでいいかも）
const CGB_MODE_NONE: u8 = 0xFF;

// [CGB対応]
// TODO :MBC1（GB/GBC共通） ... テリーのワンダーランド
// TODO :MBC3（GB/GBC共通） ... ポケモン（金、銀）
// TODO :MBC3 (GBC専用)     ... ポケモン（クリスタル）
// TODO :MBC5（GB/GBC共通） ... DQ1&2、ゼルダ夢をみる島DX
// TODO :MBC5 (GBC専用)     ... DQ3、マリオDX

// [リファレンス]
// https://gbdev.io/pandocs/CGB_Registers.html
// ↓レジスタ一覧
// https://gbdev.io/pandocs/Hardware_Reg_List.html?highlight=bcps#hardware-registers
#[allow(dead_code)]
pub struct CGB {
    hdma1: u8,          // (Addr $FF51 W) VRAM DMA ソース（上位、下位） [書き込みのみ]
    hdma2: u8,          // (Addr $FF52 W) VRAM DMA ソース（上位、下位） [書き込みのみ]
    hdma3: u8,          // (Addr $FF53 W) VRAM DMA 宛先（上位、下位） [書き込みのみ]
    hdma4: u8,          // (Addr $FF54 W) VRAM DMA 宛先（上位、下位） [書き込みのみ]
    hdma5: u8,          // (Addr $FF55 W) VRAM DMA 長さ/モード/開始
    vbk: u8,            // (Addr $FF4F R/W) VRAM バンク
    key_1: u8,          // (Addr $FF4D R/W) スピードスイッチの準備
    rp: u8,             // (Addr $FF56 R/W) 赤外線通信ポート
    opri: u8,           // (Addr $FF6C R/W) オブジェクト優先モード
    svbk: u8,           // (Addr $FF70 R/W) WRAM バンク
    bcps: u8,           // (Addr $FF68 R/W) 背景色パレット仕様
    bgpi: u8,           // (Addr $FF68 R/W) 背景パレット インデックス
    bcpd: u8,           // (Addr $FF69 R/W) 背景色パレットデータ
    bgpd: u8,           // (Addr $FF69 R/W) 背景パレットデータ
    ocps: u8,           // (Addr $FF6A R/W) OBJ カラーパレット仕様
    obpi: u8,           // (Addr $FF6A R/W) OBJ パレットインデックス
    ocpd: u8,           // (Addr $FF6B R/W) OBJ カラーパレットデータ
    obpd: u8,           // (Addr $FF6B R/W) OBJ パレットデータ
    pcm12: u8,          // (Addr $FF76 R) Audio digital outputs 1 & 2
    pcm34: u8,          // (Addr $FF77 R) Audio digital outputs 3 & 4

    unlock_flg: bool,   // アンロックフラグ
    cgb_mode: u8,       // CGBモード
}

impl CGB {
    pub fn new() -> Self {
        CGB {
            hdma1: 0,
            hdma2: 0,
            hdma3: 0,
            hdma4: 0,
            hdma5: 0,
            vbk: 0,
            key_1: 0,
            rp: 0,
            opri: 0,
            svbk: 0,
            bcps: 0,
            bgpi: 0,
            bcpd: 0,
            bgpd: 0,
            ocps: 0,
            obpi: 0,
            ocpd: 0,
            obpd: 0,
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
}

#[allow(dead_code)]
impl IO for CGB {
    fn write(&mut self, addr: u16, val: u8) {
        match addr {
            0xFF4D => self.key_1 = val,
            0xFF4F => self.vbk = val,
            0xFF51 => self.hdma1 = val,
            0xFF52 => self.hdma2 = val,
            0xFF53 => self.hdma3 = val,
            0xFF54 => self.hdma4 = val,
            0xFF55 => self.hdma5 = val,
            0xFF56 => self.rp = val,
            0xFF68 => self.bcps = val,
            0xFF69 => self.bcpd = val,
            0xFF6A => self.ocps = val,
            0xFF6B => self.ocpd = val,
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
            0xFF68 => self.bcps,
            0xFF69 => self.bcpd,
            0xFF6A => self.ocps,
            0xFF6B => self.ocpd,
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