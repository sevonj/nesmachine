use super::bus::Bus;

#[derive(Debug)]
pub struct Ppu {
    framebuffer: [u8; 256 * 240 * 3],

    scanline: usize,
    /// Within scanline
    cycle: usize,

    /// VRAM addr
    _v: u16,
    /// Temp VRAM addr
    _t: u16,
    /// Fine x scroll (3 bits)
    _x: u8,
    /// Write
    _w: bool,

    frame_even: bool,

    pattern_tile_idx: u16,
}

impl Default for Ppu {
    fn default() -> Self {
        Self {
            framebuffer: [0; 256 * 240 * 3],

            scanline: 0,
            cycle: 0,
            _v: 0,
            _t: 0,
            _x: 0,
            _w: false,
            frame_even: true,
            pattern_tile_idx: 0,
        }
    }
}

impl Ppu {
    /// Step one PPU tick (3 CPU cycles)
    /// (PAL does 3.2, not implemented)
    pub fn step(&mut self, bus: &mut Bus) {
        cpu_ppu_access(bus);

        match self.scanline {
            0..=239 => self.process_render_scanline(bus),
            240 => (), // Post-render idle
            241 => self.process_vblank_set_scanline(bus),
            242..=260 => (), // Vblank idle
            261 => self.process_pre_render_scanline(bus),
            _ => unreachable!(),
        }

        self.cycle += 1;
        if self.cycle == 341 {
            self.cycle = 0;
            self.scanline += 1;
        }
        if self.scanline == 262 {
            self.scanline = 0;
            self.frame_even = !self.frame_even;
        }
    }

    fn process_pre_render_scanline(&mut self, _bus: &Bus) {}

    fn process_render_scanline(&mut self, bus: &mut Bus) {
        // PPU skips first idle cycle on even frames
        if self.cycle == 0 && self.frame_even {
            self.cycle += 1;
        }

        match self.cycle {
            0 => (),
            1..=256 => {
                // BG
                match (self.cycle - 1) % 8 {
                    1 => self.fetch_nametable(bus),
                    3 => self.fetch_attribute(bus),
                    5 => self.fetch_pattern_lo(bus),
                    7 => self.fetch_pattern_hi(bus),
                    _ => (),
                }

                self.render_pixel(bus);
            }
            257..=320 => {
                //
            }
            321..=336 => {
                //
            }
            337..=340 => {
                //
            }
            _ => unreachable!(),
        }
    }

    fn process_vblank_set_scanline(&mut self, bus: &mut Bus) {
        if self.cycle == 1 {
            bus.ppu_regs.vblank = true;
        }
    }

    fn fetch_nametable(&mut self, _bus: &mut Bus) {
        //
    }

    fn fetch_attribute(&mut self, _bus: &mut Bus) {
        //
    }

    fn fetch_pattern_lo(&mut self, _bus: &mut Bus) {
        self.pattern_tile_idx &= 0xff00;
        //let value = bus.read_ppu(address) as u16;
        //self.pattern_tile_idx |= value;
    }

    fn fetch_pattern_hi(&mut self, _bus: &mut Bus) {
        self.pattern_tile_idx &= 0x00ff;
        //let value = bus.read_ppu(address + 1) as u16;
        //self.pattern_tile_idx |= value << 8;
    }

    fn render_pixel(&mut self, bus: &mut Bus) {
        let pixel_idx = self.cycle - 1 + self.scanline * 256;
        let pixel_off = pixel_idx * 3;

        let name_idx = (self.cycle as u16 - 1) / 8 + (self.scanline as u16 / 8) * 32;
        let name_off = bus.ppu_regs.base_nametable_addr + name_idx;
        let tile_idx = bus.read_ppu(name_off);
        //println!("{tile_idx:02X}");

        let color = tile_idx;
        // Placeholder palette
        //let color = match self.cycle % 4 {
        //    // pattern[i] {
        //    0 => 0x00,
        //    1 => 0x0f,
        //    2 => 0xf0,
        //    3 => 0xff,
        //    _ => unreachable!(),
        //};

        self.framebuffer[pixel_off] = color;
        self.framebuffer[pixel_off + 1] = color;
        self.framebuffer[pixel_off + 2] = color;
    }

    //fn read_pattern(&self, table: u8, index: u8, slice: u8) -> [u8; 8] {
    //todo!()
    //let address = self.pattern_tile_idx;
    //let lo = self.read(address);
    //let hi = self.read(address + 8);
    //let mut pattern = [0u8; 8];
    //for x in 0..8 {
    //    pattern[7 - x] = ((lo >> x) & 1) | (((hi >> x) << 1) & 2);
    //}
    //pattern
    //}

    pub fn framebuffer(&self) -> &[u8; 256 * 240 * 3] {
        &self.framebuffer
    }

    pub fn scanline(&self) -> usize {
        self.scanline
    }

    pub fn cycle(&self) -> usize {
        self.cycle
    }

    /// Reset button behavior
    pub fn reset(&mut self) {
        self.frame_even = true;
    }
}

fn cpu_ppu_access(bus: &mut Bus) {
    let addr = bus.ppu_regs.ppu_addr;
    let value = bus.ppu_regs.ppu_read_buf;

    if bus.ppu_regs.ppu_read_refresh {
        bus.ppu_regs.ppu_read_buf = bus.read_ppu(addr);
    }
    if bus.ppu_regs.ppu_written {
        bus.write_ppu(addr, value);
        println!("w {addr:04x} {value:02x}");
    }

    bus.ppu_regs.ppu_read_refresh = false;
    bus.ppu_regs.ppu_written = false;
}
