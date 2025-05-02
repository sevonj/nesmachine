use super::{CpuDevice, Device};

/// This is inside the bus to make it accessible to both CPU and PPU.
/// PPU is not here so it can access devices through the bus.
#[derive(Debug)]
pub struct PpuRegisters {
    // /-- PPU CTRL register
    /// Vblank interrupt
    pub nmi_enable: bool,
    /// Unused and unusable PPU slave mode
    pub ppu_slave: bool,
    /// Use double height sprites instead of 8x8.
    pub sprite_8x16: bool,
    /// Possibile values are 0x0000, 0x1000
    pub base_bg_pattern_addr: u16,
    /// Possibile values are 0x0000, 0x1000
    pub base_sprite_pattern_addr: u16,
    /// If set, vram access increments addr by 32. If unset, just 1.
    pub vram_big_increment: bool,
    /// Possibile values are 0x2000, 0x2400, 0x2800, 0x2c00
    pub base_nametable_addr: u16,
    // PPU CTRL --/
    pub ppu_mask: u8,

    // PPU status --/
    pub vblank: bool,
    pub sprite_0_hit: bool,
    pub sprite_overflow: bool,

    pub oam_addr: u8,
    pub oam_data: u8,

    pub scroll_x: u8,
    pub scroll_y: u8,
    pub scroll_last_x: bool,

    pub ppu_addr: u16,
    pub ppu_addr_last_msb: bool,
    pub ppu_read_buf: u8,
    pub ppu_read_refresh: bool,
    pub ppu_written: bool,

    pub oam_dma: u8,
}

impl Default for PpuRegisters {
    fn default() -> Self {
        Self {
            nmi_enable: false,
            ppu_slave: false,
            sprite_8x16: false,
            base_bg_pattern_addr: 0x0000,
            base_sprite_pattern_addr: 0x0000,
            vram_big_increment: false,
            base_nametable_addr: 0x2000,

            ppu_mask: 0,

            vblank: true,
            sprite_0_hit: false,
            sprite_overflow: true,

            oam_addr: 0,
            oam_data: 0,
            scroll_x: 0,
            scroll_y: 0,
            scroll_last_x: false,
            ppu_addr: 0,
            ppu_addr_last_msb: Default::default(),
            ppu_read_buf: 0,
            ppu_read_refresh: false,
            ppu_written: false,
            oam_dma: Default::default(),
        }
    }
}

impl PpuRegisters {
    fn status(&self) -> u8 {
        let mut value = 0;
        if self.vblank {
            value |= 1 << 7;
        }
        if self.sprite_0_hit {
            value |= 1 << 6;
        }
        if self.sprite_overflow {
            value |= 1 << 5;
        }
        value
    }

    /// Reading clears vblank flag
    fn status_destructive(&mut self) -> u8 {
        let status = self.status();
        self.vblank = false;
        status
    }

    fn read_oamdata(&self) -> u8 {
        0
    }

    fn read_vram(&mut self) -> u8 {
        self.inc_vram_addr();
        self.ppu_read_refresh = true;
        self.ppu_read_buf
    }

    fn write_ppuctrl(&mut self, value: u8) {
        self.nmi_enable = value & 0x80 != 0;
        self.ppu_slave = value & 0x40 != 0;
        self.sprite_8x16 = value & 0x20 != 0;
        self.base_bg_pattern_addr = if value & 0x10 == 0 { 0x0000 } else { 0x1000 };
        self.base_sprite_pattern_addr = if value & 0x08 == 0 { 0x0000 } else { 0x1000 };
        self.vram_big_increment = value & 0x04 != 0;
        self.base_nametable_addr = match value & 0x03 {
            0 => 0x2000,
            1 => 0x2400,
            2 => 0x2800,
            3 => 0x2c00,
            _ => unreachable!(),
        }
    }

    fn write_ppumask(&mut self, value: u8) {
        self.ppu_mask = value;
    }

    fn write_oamaddr(&mut self, value: u8) {
        self.oam_addr = value;
    }

    fn write_oamdata(&mut self, _value: u8) {}

    fn write_ppuscroll(&mut self, value: u8) {
        if self.scroll_last_x {
            self.scroll_y = value;
        } else {
            self.scroll_x = value;
        }
        self.scroll_last_x = !self.scroll_last_x;
    }

    fn write_ppuaddr(&mut self, value: u8) {
        let value = value as u16;
        if self.ppu_addr_last_msb {
            self.ppu_addr &= 0x00ff;
            self.ppu_addr += value;
        } else {
            self.ppu_addr &= 0xff00;
            self.ppu_addr += value << 8;
        }

        self.ppu_addr_last_msb = !self.ppu_addr_last_msb;
    }

    fn write_vram(&mut self, value: u8) {
        self.inc_vram_addr();
        self.ppu_read_buf = value;
        self.ppu_written = true;
    }

    fn inc_vram_addr(&mut self) {
        self.ppu_addr += if self.vram_big_increment { 32 } else { 1 };
    }

    /// OAM DMA hi addr
    fn write_oamdma(&self, _value: u8) {}
}

impl Device for PpuRegisters {
    fn reset(&mut self) {
        self.scroll_last_x = false;
    }
}

impl CpuDevice for PpuRegisters {
    fn read(&mut self, addr: u16) -> u8 {
        if let 0x2000..=0x3fff = addr {
            match addr % 8 {
                0x02 => self.status_destructive(),
                0x04 => self.read_oamdata(),
                0x07 => self.read_vram(),
                _ => 0,
            }
        } else {
            0
        }
    }

    fn read_immutable(&self, addr: u16) -> u8 {
        if let 0x2000..=0x3fff = addr {
            match addr % 8 {
                0x02 => self.status(),
                0x04 => self.read_oamdata(),
                0x07 => self.ppu_read_buf,
                _ => 0,
            }
        } else {
            0
        }
    }

    fn write(&mut self, addr: u16, value: u8) {
        if addr == 0x4014 {
            self.write_oamdma(value);
            return;
        }

        if let 0x2000..=0x3fff = addr {
            match addr % 8 {
                0x00 => self.write_ppuctrl(value),
                0x01 => self.write_ppumask(value),
                0x03 => self.write_oamaddr(value),
                0x04 => self.write_oamdata(value),
                0x05 => self.write_ppuscroll(value),
                0x06 => self.write_ppuaddr(value),
                0x07 => self.write_vram(value),
                _ => (),
            }
        }
    }
}
