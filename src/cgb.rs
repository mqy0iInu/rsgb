// use common::*;

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
#[allow(dead_code)]
pub struct CGB {
    hdma1: u8,          // VRAM DMA ソース（上位、下位） [書き込みのみ]
    hdma2: u8,          // VRAM DMA ソース（上位、下位） [書き込みのみ]
    hdma3: u8,          // VRAM DMA 宛先（上位、下位） [書き込みのみ]
    hdma4: u8,          // VRAM DMA 宛先（上位、下位） [書き込みのみ]
    hdma5: u8,          // VRAM DMA 長さ/モード/開始
    vbk: u8,            // VRAM バンク
    key_1: u8,          // スピードスイッチの準備
    rp: u8,             // 赤外線通信ポート
    opri: u8,           // オブジェクト優先モード
    svbk: u8,           // WRAM バンク

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

    pub fn vbk_write(&mut self, val: u8) {
        self.vbk = val;
    }

    pub fn svbk_write(&mut self, val: u8) {
        self.svbk = val;
    }
}

#[allow(dead_code)]
pub struct CgbPalette {
    bcps: u8, // 背景色パレット仕様
    bgpi: u8, // 背景パレット インデックス
    bcpd: u8, // 背景色パレットデータ
    bgpd: u8, // 背景パレットデータ
    ocps: u8, // OBJ カラーパレット仕様
    obpi: u8, // OBJ パレットインデックス
    ocpd: u8, // OBJ カラーパレットデータ
    obpd: u8, // OBJ パレットデータ
}

impl  CgbPalette {
    pub fn new() -> Self {
        CgbPalette {
            bcps: 0,
            bgpi: 0,
            bcpd: 0,
            bgpd: 0,
            ocps: 0,
            obpi: 0,
            ocpd: 0,
            obpd: 0,
        }
    }
}