use common::*;
// use  cgb::*;
use bitvec::prelude::*;

// const VRAM_SIZE: usize = 8 * 1024;  // DMG
const VRAM_SIZE: usize = 32 * 1024; // CGB (8KB * 2バンク)
const VRAM_BANK_SIZE: u16 = 8 * 1024;
const OAM_SIZE: usize = 0xA0;

// For DMG　モノクロパレット
const COLOR_LIGHT_GRAY: u8 = 0xAA;
const COLOR_DARK_GRAY: u8 = 0x55;
const COLOR_BLACK: u8 = 0x00;
const COLOR_WHITE: u8 = 0xFF;

// [VRAM Mem Map]
// **********************************
// Bank 0
// **********************************
// 0x8000-0x87FF: Tile Set #1
// 0x8800-0x8FFF: Tile Set #2
// 0x9000-0x97FF: Tile Set #3
// 0x9800-0x9BFF: Tile Map #1
// 0x9C00-0x9FFF: Tile Map #2
// **********************************
// Bank 1 (CBG Only)
// **********************************
// 0x8000-0x87FF: (調査中)
// 0x8800-0x8FFF: (調査中)
// 0x9000-0x97FF: (調査中)
// 0x9800-0x9BFF: BG Map Attributes (※注)
// 0x9C00-0x9FFF: (調査中)
// **********************************
// (※注)は@get_bg_plt_num()に記載

pub const _BG_MAP_ATTRIBUTE_SIZE: usize = 32 * 32;


#[derive(Default, Clone, PartialEq, Eq)]
pub struct RGB24Color {
    pub r: u8,
    pub g: u8,
    pub b: u8,
}

#[derive(Copy, Clone, PartialEq)]
enum BGPriority {
    Color0,
    Color123,
}

#[allow(dead_code)]
pub struct PPU {
    vram: [u8; VRAM_SIZE],            // VRAM
    oam: [u8; OAM_SIZE],              // OAM
    lcdc: u8,                         // LCD Control
    stat: u8,                         // Status
    scy: u8,                          // Scroll Y
    scx: u8,                          // Scroll X
    ly: u8,                           // Y-Coordinate
    lyc: u8,                          // LY Compare
    dma: u8,                          // DMA Transfer and Start Address
    bgp: u8,                          // Background Palette Data
    obp0: u8,                         // Object Palette 0 Data
    obp1: u8,                         // Object Palette 1 Data
    wy: u8,                           // Window Y Position
    wx: u8,                           // Window X Position minus 7
    pub irq_vblank: bool,             // V-Blank interrupt request
    pub irq_lcdc: bool,               // LCDC interrupt request
    cnt: u16,                         // Elapsed clocks in current mode
    bg_prio: [BGPriority; SCREEN_W as usize],  // Background priority


    // DMG
    frame_buffer: [u8; SCREEN_WH],    // Frame buffer
    scanline: [u8; SCREEN_W as usize], // Current scanline

    // TODO CGBの時だけインスタンス生成
    // CGB
    frame_buffer_col: [u16; SCREEN_WH],  // CGB対応　カラーフレームバッファ(u16 ... RGB555)
    scanline_col: [u16; SCREEN_W as usize], // CGB対応　カラースキャンライン(u16 ... RGB555)
    pub cgb_mode: u8,                  // CGB動作モード (CGB Only)
    pub cgb_unlock_flg: bool,          // CGB動作フラグ (CGB Only)
    pub vram_bank: u8,                 // VRAM バンク (CGB Only)
    pub p_bg_col_plt: *const u8,       // BGカラーパレットポインタ(CGB Only)
    pub p_obj_col_plt: *const u8,      // OBJカラーパレットポインタ(CGB Only)
}

impl PPU {
    pub fn new(p_bg_col_plt: *const u8, p_obj_col_plt: *const u8) -> Self {
        PPU {
            vram: [0; VRAM_SIZE ],
            oam: [0; OAM_SIZE],
            lcdc: 0x80,
            stat: 0x02,
            scy: 0,
            scx: 0,
            ly: 0,
            lyc: 0,
            dma: 0,
            bgp: 0,
            obp0: 0,
            obp1: 0,
            wy: 0,
            wx: 0,
            irq_vblank: false,
            irq_lcdc: false,
            cnt: 0,
            bg_prio: [BGPriority::Color0; SCREEN_W as usize],

            // DMG
            scanline: [0; SCREEN_W as usize],
            frame_buffer: [0; SCREEN_WH],

            // CGB
            scanline_col: [0; SCREEN_W as usize],
            frame_buffer_col: [0; SCREEN_WH],
            cgb_mode: 0,
            cgb_unlock_flg: false,
            vram_bank: 0,
            p_bg_col_plt: p_bg_col_plt,
            p_obj_col_plt: p_obj_col_plt,
        }
    }

    // (DMG, 非CGBモード専用): モノクロパレットの色変換
    // https://gbdev.io/pandocs/Palettes.html#ff47--bgp-non-cgb-mode-only-bg-palette-data
    fn map_color(&self, color_num: u8, palette: u8) -> u8 {
        match (palette >> (color_num << 1)) & 0x03 {
            0 => COLOR_WHITE,        // 白
            1 => COLOR_LIGHT_GRAY,   // ライトグレー
            2 => COLOR_DARK_GRAY,    // ダークグレー
            3 | _ => COLOR_BLACK,    // 黒
        }
    }

    // (※注)
    // BG Map Attributesについて理解したので翻訳メモ（https://gbdev.io/pandocs/Tile_Maps.html）
    //
    // [メモ] BG/OBJカラーパレット0~7のどのパレットを指定する方法は？
    // PRG-ROMに仕込まれたBG Map AttributesがVRAM Bank1 $9C00~$9FFFにロードさせる
    // サイズは32x32Byte(1024KB)で、下記のような構成でBit0がパレットの指定になる
    // ★VRAM Bank0 @ $9800~$9BFFは、VRAM Bank1 @ $9800~$9BFFが対応して色付けしてる、ここ重要！
    // [BG Map Attributesの構成(これが32x32ある)]
    // ================================================================================
    // Bit 7    BG-to-OAM Priority         (0=Use OAM Priority bit, 1=BG Priority)
    // Bit 6    Vertical Flip              (0=Normal, 1=Mirror vertically)
    // Bit 5    Horizontal Flip            (0=Normal, 1=Mirror horizontally)
    // Bit 4    Not used
    // Bit 3    Tile VRAM Bank number      (0=Bank 0, 1=Bank 1)
    // Bit 2-0  Background Palette number  (BGP0-7)
    // ================================================================================
    #[allow(dead_code)]
    fn get_bg_plt_num(&mut self, addr: u16) -> u8
    {
        // TODO BG Map AttributeからBGパレット番号の取得処理
        let offset = VRAM_BANK_SIZE as usize * self.vram_bank as usize;
        let bg_plt_num: u8 = self.vram[addr as usize + offset] & 0x07; // 0x07 = Bit 2-0
        bg_plt_num
    }

    // (GB/GBC共通、GBC専用) カラーパレット(RGB555)のRGB24変換
    #[allow(dead_code)]
    fn rgb555_to_rgb24(&mut self, rgb555: u16) -> RGB24Color {
        let tmp = rgb555.view_bits::<Lsb0>();
        let r = tmp[0..=4].load::<u8>();
        let g = tmp[5..=9].load::<u8>();
        let b = tmp[10..=14].load::<u8>();
        RGB24Color {
            r: r << 3 | r >> 2,
            g: g << 3 | g >> 2,
            b: b << 3 | b >> 2,
        }
    }

    // Fetches tile data from VRAM.
    fn fetch_tile(&self, tile_no: u8, offset_y: u8, tile_data_sel: bool) -> (u8, u8) {
        // Fetch tile data from tile set
        let tile_data_addr = if tile_data_sel {
            // Use tile set #1 (0x0000-0x07ff) and #2 (0x0800-0x0fff)
            (tile_no as u16) << 4
        } else {
            // Use tile set #2 (0x0800-0x0fff) and #3 (0x1000-0x17ff)
            (0x1000 as u16).wrapping_add(((tile_no as i8 as i16) << 4) as u16)
        };
        let row_addr = tile_data_addr + (offset_y << 1) as u16;

        let mut _tile0: u8 = 0;
        let mut _tile1: u8 = 0;
        if self.cgb_unlock_flg != false {
            let offset = VRAM_BANK_SIZE as usize * self.vram_bank as usize;
            _tile0 = self.vram[row_addr as usize + offset];
            _tile1 = self.vram[(row_addr + 1) as usize + offset];
        }else{
            _tile0 = self.vram[row_addr as usize];
            _tile1 = self.vram[(row_addr + 1) as usize];
        }

        (_tile0, _tile1)
    }

    // Fetches BG or Window tile data from VRAM.
    fn fetch_bg_window_tile(
        &self,
        tile_x: u8,
        tile_y: u8,
        offset_y: u8,
        tile_map_base: u16,
    ) -> (u8, u8) {
        // Fetch tile index from tile map
        let tile_map_addr = tile_map_base | ((tile_x & 0x1F) as u16 + ((tile_y as u16) << 5));

        let mut _tile_no: u8 = 0;
        if self.cgb_unlock_flg != false {
            let offset = VRAM_BANK_SIZE as usize * self.vram_bank as usize;
            _tile_no = self.vram[tile_map_addr as usize + offset];
        }else{
            _tile_no = self.vram[tile_map_addr as usize];
        }

        self.fetch_tile(_tile_no, offset_y, self.lcdc & 0x10 > 0)
    }

    // Fetches BG tile data from VRAM.
    fn fetch_bg_tile(&self, tile_x: u8, tile_y: u8, offset_y: u8) -> (u8, u8) {
        // Fetch tile index from tile map
        let tile_map_base = if self.lcdc & 0x8 > 0 { 0x1C00 } else { 0x1800 };

        self.fetch_bg_window_tile(tile_x, tile_y, offset_y, tile_map_base)
    }

    // Fetches Window tile data from VRAM.
    fn fetch_window_tile(&self, tile_x: u8, tile_y: u8, offset_y: u8) -> (u8, u8) {
        // Fetch tile index from tile map
        let tile_map_base = if self.lcdc & 0x40 > 0 { 0x1C00 } else { 0x1800 };

        self.fetch_bg_window_tile(tile_x, tile_y, offset_y, tile_map_base)
    }

    // Returns the color number at a given position from tile data.
    fn get_color_num(&self, tile: (u8, u8), bitpos: u8) -> u8 {
        let lo_bit = tile.0 >> bitpos & 1;
        let hi_bit = tile.1 >> bitpos & 1;

        hi_bit << 1 | lo_bit
    }

    // Renders BG.
    fn render_bg(&mut self) {
        // Tile coordinate
        let mut tile_x = self.scx >> 3;
        let mut tile_y = self.scy.wrapping_add(self.ly) >> 3;

        // Offset of current pixel within tile
        let mut offset_x = self.scx & 0x7;
        let mut offset_y = self.scy.wrapping_add(self.ly) & 0x7;

        let mut tile = self.fetch_bg_tile(tile_x, tile_y, offset_y);

        let mut window = false;

        for x in 0..SCREEN_W {
            // Check if window is enabled
            if self.lcdc & 0x20 > 0 {
                if self.wy <= self.ly && self.wx == x + 7 {
                    tile_x = 0;
                    tile_y = (self.ly - self.wy) >> 3;
                    offset_x = 0;
                    offset_y = (self.ly - self.wy) & 0x7;
                    tile = self.fetch_window_tile(tile_x, tile_y, offset_y);
                    window = true;
                }
            }

            let color_num = self.get_color_num(tile, 7 - offset_x);
            let color = self.map_color(color_num, self.bgp);

            self.bg_prio[x as usize] = if color_num == 0 {
                BGPriority::Color0
            } else {
                BGPriority::Color123
            };

            self.scanline[x as usize] = color;

            offset_x += 1;

            // Move on to next tile
            if offset_x >= 8 {
                offset_x = 0;
                tile_x += 1;

                if window {
                    tile = self.fetch_window_tile(tile_x, tile_y, offset_y);
                } else {
                    tile = self.fetch_bg_tile(tile_x, tile_y, offset_y);
                }
            }
        }
    }

    // Renders sprites.
    fn render_sprites(&mut self) {
        let mut n_sprites = 0;
        let height = if self.lcdc & 0x4 > 0 { 16 } else { 8 };

        for i in 0..40 {
            // Parse OAM entry
            let entry_addr = i << 2;
            let sprite_y = self.oam[entry_addr];
            let sprite_x = self.oam[entry_addr + 1];
            let flags = self.oam[entry_addr + 3];

            let obj_prio = flags & 0x80 > 0;
            let flip_y = flags & 0x40 > 0;
            let flip_x = flags & 0x20 > 0;
            let palette = if flags & 0x10 > 0 {
                self.obp1
            } else {
                self.obp0
            };

            // Check if sprite is visible on this scanline
            if sprite_y <= self.ly + 16 - height || sprite_y > self.ly + 16 {
                continue;
            }

            // Up to 10 sprites can be rendered on one scanline
            n_sprites += 1;
            if n_sprites > 10 {
                break;
            }

            // Check if sprite is within the screen
            if sprite_x == 0 || sprite_x > SCREEN_W + 8 - 1 {
                continue;
            }

            // Tile number
            let tile_no = if self.lcdc & 0x4 > 0 {
                // 8x16 sprite
                if (self.ly + 8 < sprite_y) ^ flip_y {
                    self.oam[entry_addr + 2] & 0xfe
                } else {
                    self.oam[entry_addr + 2] | 0x01
                }
            } else {
                // 8x8 sprite
                self.oam[entry_addr + 2]
            };

            // Y-offset within the tile
            let offset_y = if flip_y {
                7 - ((self.ly + 16 - sprite_y) & 0x7)
            } else {
                (self.ly + 16 - sprite_y) & 0x7
            };

            // Fetch tile data
            let tile = self.fetch_tile(tile_no, offset_y, true);

            for offset_x in 0..8 {
                if offset_x + sprite_x < 8 {
                    continue;
                }

                let x = offset_x + sprite_x - 8;

                if x >= SCREEN_W {
                    break;
                }

                let bitpos = if flip_x { offset_x } else { 7 - offset_x };
                let color_num = self.get_color_num(tile, bitpos);
                if color_num == 0 {
                    continue;
                }
                if self.bg_prio[x as usize] == BGPriority::Color123 && obj_prio {
                    continue;
                }
                let color = self.map_color(color_num, palette);

                self.scanline[x as usize] = color;
            }
        }
    }

    // Renders a scanline.
    fn render_scanline(&mut self) {
        if self.lcdc & 0x1 > 0 {
            self.render_bg();
        }
        if self.lcdc & 0x2 > 0 {
            self.render_sprites();
        }

        for x in 0..SCREEN_W {
            let ix = (x as usize) + (self.ly as usize) * (SCREEN_W as usize);
            self.frame_buffer[ix] = self.scanline[x as usize];
        }
    }

    // Returns the current contents of the frame buffer.
    pub fn frame_buffer(&self) -> &[u8] {
        &self.frame_buffer
    }

    // Checks LYC interrupt.
    fn update_lyc_interrupt(&mut self) {
        // LYC=LY coincidence interrupt
        if self.ly == self.lyc {
            self.stat |= 0x4;

            if self.stat & 0x40 > 0 {
                self.irq_lcdc = true;
            }
        } else {
            self.stat &= !0x4;
        }
    }

    // Checks LCD mode interrupt.
    fn update_mode_interrupt(&mut self) {
        // Mode interrupts
        match self.stat & 0x03 {
            // H-Blank interrupt
            0 if self.stat & 0x8 > 0 => self.irq_lcdc = true,
            // V-Blank interrupt
            1 if self.stat & 0x10 > 0 => self.irq_lcdc = true,
            // OAM Search interrupt
            2 if self.stat & 0x20 > 0 => self.irq_lcdc = true,
            _ => (),
        }
    }
}

impl IO for PPU {
    fn write(&mut self, addr: u16, val: u8) {
        match addr {
            // VRAM
            0x8000..=0x9FFF => {
                // VRAM is inaccessible during pixel transfer
                if self.stat & 0x03 != 3 {
                    if self.cgb_unlock_flg != false {
                        let offset = VRAM_BANK_SIZE as usize * self.vram_bank as usize;
                        self.vram[(addr & 0x1FFF) as usize + offset] = val
                    }else{
                        self.vram[(addr & 0x1FFF) as usize] = val
                    }
                }
            }

            // OAM
            0xFE00..=0xFE9F => {
                // OAM is only accessible during H-Blank and V-Blank
                if self.stat & 0x03 == 0 || self.stat & 0x03 == 1 {
                    self.oam[(addr & 0x00FF) as usize] = val;
                }
            }

            // I/O registers
            0xFF40 => {
                if self.lcdc & 0x80 != val & 0x80 {
                    self.ly = 0;
                    self.cnt = 0;

                    let mode = if val & 0x80 > 0 { 2 } else { 0 };
                    self.stat = (self.stat & 0xF8) | mode;
                    self.update_mode_interrupt();
                }

                self.lcdc = val;
            }
            0xFF41 => self.stat = (val & 0xF8) | (self.stat & 0x03),
            0xFF42 => self.scy = val,
            0xFF43 => self.scx = val,
            0xFF44 => (),
            0xFF45 => {
                if self.lyc != val {
                    self.lyc = val;
                    self.update_lyc_interrupt();
                }
            }
            0xFF47 => self.bgp = val,
            0xFF48 => self.obp0 = val,
            0xFF49 => self.obp1 = val,
            0xFF4A => self.wy = val,
            0xFF4B => self.wx = val,

            _ => unreachable!("Unexpected address: 0x{:04X}", addr),
        }
    }

    fn read(&mut self, addr: u16) -> u8 {
        match addr {
            // VRAM
            0x8000..=0x9FFF => {
                // VRAM is inaccessible during pixel transfer
                if self.stat & 0x03 != 3 {
                    if self.cgb_unlock_flg != false {
                        let offset = VRAM_BANK_SIZE as usize * self.vram_bank as usize;
                        self.vram[(addr & 0x1FFF) as usize + offset]
                    }else{
                        self.vram[(addr & 0x1FFF) as usize]
                    }
                } else {
                    0xFF
                }
            }

            // OAM
            0xFE00..=0xFE9F => {
                // OAM is only accessible during H-Blank and V-Blank
                if self.stat & 0x03 == 0 || self.stat & 0x03 == 1 {
                    self.oam[(addr & 0x00ff) as usize]
                } else {
                    0xFF
                }
            }

            // IO registers
            0xFF40 => self.lcdc,
            0xFF41 => self.stat,
            0xFF42 => self.scy,
            0xFF43 => self.scx,
            0xFF44 => self.ly,
            0xFF45 => self.lyc,
            0xFF46 => self.dma,
            0xFF47 => self.bgp,
            0xFF48 => self.obp0,
            0xFF49 => self.obp1,
            0xFF4A => self.wy,
            0xFF4B => self.wx,

            _ => unreachable!("Unexpected address: 0x{:04X}", addr),
        }
    }

    fn update(&mut self, tick: u8) {
        if self.lcdc & 0x80 == 0 {
            return;
        }

        self.cnt += tick as u16;

        match self.stat & 0x03 {
            // OAM Search (80 clocks)
            2 => {
                if self.cnt >= 80 {
                    self.cnt -= 80;
                    // Transition to Pixel Transfer mode
                    self.stat = (self.stat & 0xF8) | 3;
                    self.render_scanline();
                }
            }
            // Pixel Transfer (172 clocks)
            3 => {
                if self.cnt >= 172 {
                    self.cnt -= 172;
                    // Transition to H-Blank mode
                    self.stat = self.stat & 0xF8;
                    self.update_mode_interrupt();
                }
            }
            // H-Blank (204 clocks)
            0 => {
                if self.cnt >= 204 {
                    self.cnt -= 204;
                    self.ly += 1;

                    if self.ly >= SCREEN_H {
                        // Transition to V-Blank mode
                        self.stat = (self.stat & 0xF8) | 1;
                        self.irq_vblank = true;
                    } else {
                        // Transition to OAM Search mode
                        self.stat = (self.stat & 0xF8) | 2;
                    }

                    self.update_lyc_interrupt();
                    self.update_mode_interrupt();
                }
            }
            // V-Blank (4560 clocks or 10 lines)
            1 | _ => {
                if self.cnt >= 456 {
                    self.cnt -= 456;
                    self.ly += 1;

                    if self.ly >= 154 {
                        // Transition to OAM Search mode
                        self.stat = (self.stat & 0xF8) | 2;
                        self.ly = 0;

                        self.update_mode_interrupt();
                    }

                    self.update_lyc_interrupt();
                }
            }
        }
    }
}
