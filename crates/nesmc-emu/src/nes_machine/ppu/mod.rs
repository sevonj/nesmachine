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
    pub nmi_fired: bool,
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
            nmi_fired: false,
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
            if bus.ppu_regs.ctrl.nmi_enable {
                self.nmi_fired = true;
            }
        }
    }

    // TODO: Cycle-accurate rendering
    fn fetch_nametable(&mut self, _bus: &mut Bus) {
        // let current_px_idx = self.cycle - 1 + self.scanline * 256;
        // let fetch_px_idx = (current_px_idx + 16) % self.framebuffer.len();
    }

    // TODO: Cycle-accurate rendering
    fn fetch_attribute(&mut self, _bus: &mut Bus) {
        //
    }

    // TODO: Cycle-accurate rendering
    fn fetch_pattern_lo(&mut self, _bus: &mut Bus) {
        self.pattern_tile_idx &= 0xff00;
        //let value = bus.read_ppu(address) as u16;
        //self.pattern_tile_idx |= value;
    }

    // TODO: Cycle-accurate rendering
    fn fetch_pattern_hi(&mut self, _bus: &mut Bus) {
        self.pattern_tile_idx &= 0x00ff;
        //let value = bus.read_ppu(address + 1) as u16;
        //self.pattern_tile_idx |= value << 8;
    }

    // TODO: Cycle-accurate rendering
    fn render_pixel(&mut self, bus: &mut Bus) {
        let name_idx = (self.cycle as u16 - 1) / 8 + (self.scanline as u16 / 8) * 32;
        let name_off = bus.ppu_regs.ctrl.base_nametable_addr + name_idx;
        let tile_idx = bus.read_ppu(name_off) as u16;
        let tile_addr = tile_idx * 0x10 + bus.ppu_regs.ctrl.base_bg_pattern_addr;

        let x = (self.cycle as u16 - 1) % 8;
        let y = self.scanline as u16 % 8;

        let plane_0 = bus.read_ppu(tile_addr + y);
        let plane_1 = bus.read_ppu(tile_addr + y + 8);

        let val_lo = plane_0 << x & 0x80;
        let val_hi = plane_1 << x & 0x80;

        let color_idx = (val_lo >> 7) + (val_hi >> 6);

        // Placeholder palette
        let color = match color_idx {
            0 => (0, 0, 0),
            1 => (0x40, 0x40, 0x40),
            2 => (0x80, 0x80, 0x80),
            3 => (0xff, 0xff, 0xff),
            _ => unreachable!(),
        };

        let pixel_idx = self.cycle - 1 + self.scanline * 256;
        let pixel_off = pixel_idx * 3;

        self.framebuffer[pixel_off] = color.0;
        self.framebuffer[pixel_off + 1] = color.1;
        self.framebuffer[pixel_off + 2] = color.2;
    }

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
