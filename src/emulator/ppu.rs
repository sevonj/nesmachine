const FLAG_CTRL_N: u8 = 0b0000_0011; // Nametable
const FLAG_CTRL_I: u8 = 0b0000_0100; // Vram Inc
const FLAG_CTRL_S: u8 = 0b0000_1000; // Sprite Table
const FLAG_CTRL_B: u8 = 0b0001_0000; // BG Table
const FLAG_CTRL_H: u8 = 0b0010_0000; // Sprite Size
const FLAG_CTRL_P: u8 = 0b0100_0000; // Master/Slave
const FLAG_CTRL_V: u8 = 0b1000_0000; // NMI Enable

const FLAG_STAT_O: u8 = 0b0010_0000; // Sprite Overflow
const FLAG_STAT_S: u8 = 0b0100_0000; // Sprite hit
const FLAG_STAT_V: u8 = 0b1000_0000; // Vblank

const FLAG_MASK_Gr: u8 = 0b0000_0001; // 1: grayscale
const FLAG_MASK_m: u8 = 0b0000_0010; // 1: Show background in leftmost 8 pixels of screen, 0: Hide
const FLAG_MASK_M: u8 = 0b0000_0100; // 1: Show sprites in leftmost 8 pixels of screen, 0: Hide
const FLAG_MASK_b: u8 = 0b0000_1000; // 1: Show background
const FLAG_MASK_s: u8 = 0b0001_0000; // 1: Show sprites
const FLAG_MASK_R: u8 = 0b0010_0000; // Emphasize red (green on PAL/Dendy)
const FLAG_MASK_G: u8 = 0b0100_0000; // Emphasize green (red on PAL/Dendy)
const FLAG_MASK_B: u8 = 0b1000_0000; // Emphasize blue

pub(crate) struct PPU {
    ppu_ctrl: u8,
    ppu_mask: u8,
    ppu_status: u8,
    oam_addr: u8,
    oam_data: u8,
    ppu_scroll_x: u8,
    ppu_scroll_y: u8,
    ppu_addr: u16,
    ppu_data: u8,
    oam_dma: u8,

    vram: [u8; 0x2000],
    pub(crate) chr_rom: Vec<u8>,

    ppu_scroll_latch: bool,
    ppu_addr_latch: bool,

    pub(crate) framebuffer: Vec<u8>,
}

impl PPU {
    pub(crate) fn new() -> PPU {
        PPU {
            ppu_ctrl: 0,
            ppu_mask: 0,
            ppu_status: 0b1000_0000,
            oam_addr: 0,
            oam_data: 0,
            ppu_scroll_x: 0,
            ppu_scroll_y: 0,
            ppu_addr: 0,
            ppu_data: 0,
            oam_dma: 0,

            vram: [0; 0x2000],
            chr_rom: vec![0; 0x2000],

            ppu_scroll_latch: false,
            ppu_addr_latch: false,

            framebuffer: vec![0; 0xf000],
        }
    }
    fn read(&self, addr: u16) -> u8 {
        match addr {
            0x0000..=0x1fff => self.chr_rom[addr as usize],
            0x2000..=0x3fff => self.vram[addr as usize - 0x2000],
            _ => panic!("PPU Read: invalid addr {:04x}", addr),
        }
    }
    fn write(&mut self, addr: u16, value: u8) {
        //println!("ppu write: addr: {:04x} val: {:02x}", addr, value);
        match addr {
            0x0000..=0x1fff => self.chr_rom[addr as usize] = value,
            0x2000..=0x3fff => self.vram[addr as usize - 0x2000] = value,
            _ => panic!("PPU Write: invalid addr {:04x}", addr),
        }
    }
    //fn read_u16(&self, addr: u16) -> u16 {
    //    (self.read(addr) as u16) << 8 + self.read(addr + 1) as u16
    //}
    pub(crate) fn debug_read(&mut self, addr: u16) -> u8 {
        match addr {
            0..=0x3fff => self.read(addr),
            _ => 0x69,
        }
    }
    pub(crate) fn read_ppu_status(&mut self) -> u8 {
        self.ppu_status
    }
    pub(crate) fn read_oam_data(&mut self) -> u8 {
        self.oam_data
    }
    pub(crate) fn read_ppu_data(&mut self) -> u8 {
        self.ppu_addr_latch = false;
        self.ppu_data
    }
    pub(crate) fn write_ppu_ctrl(&mut self, value: u8) {
        self.ppu_ctrl = value;
    }
    pub(crate) fn write_ppu_mask(&mut self, value: u8) {
        self.ppu_mask = value;
    }
    pub(crate) fn write_oam_addr(&mut self, value: u8) {
        self.oam_addr = value;
    }
    pub(crate) fn write_oam_data(&mut self, value: u8) {
        self.oam_data = value;
    }
    pub(crate) fn write_ppu_addr(&mut self, value: u8) {
        if self.ppu_addr_latch {
            self.ppu_addr &= 0xff00;
            self.ppu_addr |= value as u16;
            //println!("ppu_addr set: {:02x}xx", value);
        } else {
            self.ppu_addr &= 0x00ff;
            self.ppu_addr |= (value as u16) << 8;
            //println!("ppu_addr set: xx{:02x}", value);
        }
        self.ppu_addr_latch = true;
    }
    pub(crate) fn write_ppu_scroll(&mut self, value: u8) {
        if self.ppu_scroll_latch {
            self.ppu_scroll_y = value;
        } else {
            self.ppu_scroll_x = value;
        }
        self.ppu_scroll_latch = !self.ppu_scroll_latch;
    }
    pub(crate) fn write_ppu_data(&mut self, value: u8) {
        self.ppu_data = value;
        self.write(self.ppu_addr, value);
        if self.ppu_ctrl & FLAG_CTRL_I == 0 {
            self.ppu_addr = self.ppu_addr.wrapping_add(1);
        } else {
            self.ppu_addr = self.ppu_addr.wrapping_add(32);
        }
        self.ppu_addr_latch = false;
    }
    pub(crate) fn write_oam_dma(&mut self, value: u8) {
        self.oam_dma = value;
    }

    pub(crate) fn read_pattern(&self, table: u8, index: u8, slice: u8) -> [u8; 8] {
        let address = (table as u16) * 0x1000 + (index as u16) * 16 + slice as u16;
        let lo = self.read(address);
        let hi = self.read(address + 8);
        let mut pattern = [0u8; 8];
        for i in 0..8 {
            pattern[7 - i] = ((lo >> i) & 1) | (((hi >> i) << 1) & 2);
        }
        pattern
    }
    pub(crate) fn read_nametable(&mut self, table: u8, index: u16) -> u8 {
        let address = 0x2000 + (table as u16 * 0x400) + (index % 0x3ff);
        self.read(address)
    }
    pub(crate) fn read_palette(&mut self, index: u8) -> u8 {
        self.read(0x3f00 + (index % 0x20) as u16)
    }

    pub(crate) fn render(&mut self) {
        self.set_vblank(false);

        for y in 0..240 {
            for x in 0..32 {
                // Background tiles
                let tile_offset = x as u16 + y as u16 / 8 * 32;
                let tile_index = self.read_nametable(self.ppu_ctrl & 3, tile_offset);
                let patt_table = match self.ppu_ctrl & FLAG_CTRL_B {
                    0 => 0,
                    _ => 1,
                };
                let pattern = self.read_pattern(patt_table, tile_index, (y as u8) % 8);

                for i in 0..8 {
                    // Placeholder palette
                    let color = match pattern[i] {
                        0 => 0x00,
                        1 => 0x0f,
                        2 => 0xf0,
                        3 => 0xff,
                        _ => panic!("pattern wtf {}", pattern[i]),
                    };
                    self.framebuffer[y * 256 + x * 8 + i] = color;
                }
            }
            self.set_vblank(true);
        }
    }

    fn set_vblank(&mut self, yes: bool) {
        match yes {
            true => {self.ppu_status |= FLAG_STAT_V},
            false => self.ppu_status &= !FLAG_STAT_V,
        }
    }
    /*
    pub fn render_pattern_tables(&mut self) {
        // Loop through the pattern tables
        for table_num in 0..2 {
            // Loop through the rows and columns of the pattern table
            for row in 0..16 {
                for col in 0..16 {
                    // Get the address of the tile data in the pattern table
                    let tile_addr = table_num * 0x1000 + row * 16 + col;

                    // Read the tile data from the PPU
                    let mut tile_data = [0; 16];
                    for i in 0..8 {
                        let low_byte = self.read(0x1000 * table_num + 16 * (tile_addr % 256) + i);
                        let high_byte = self.read(0x1000 * table_num + 16 * (tile_addr % 256) + i + 8);
                        for j in 0..8 {
                            let pixel_value = ((low_byte >> (7 - j)) & 0x01) | (((high_byte >> (7 - j)) & 0x01) << 1);
                            tile_data[i as usize * 2 + j as usize] = pixel_value;
                        }
                    }

                    //// Draw the tile data on the screen buffer
                    //let base_x = table_num * 128 + col * 8;
                    //let base_y = row * 8;
                    //for y in 0..8 {
                    //    for x in 0..8 {
                    //        let color_index = tile_data[y * 2 + x];
                    //        let color = match color_index {
                    //            0 => 0x00000000, // Transparent
                    //            1 => 0x00FFFFFF, // White
                    //            2 => 0x00AAAAAA, // Light gray
                    //            3 => 0x00555555, // Dark gray
                    //            _ => unreachable!(),
                    //        };
                    //        self.framebuffer[(base_y + y) * 256 + base_x + x] = color;
                    //    }
                    //}
                }
            }
        }
    }*/
}
