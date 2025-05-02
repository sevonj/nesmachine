use super::PpuDevice;

/// PPU Palette RAM
#[derive(Debug)]
pub struct PRam(pub [u8; Self::SIZE]);

impl PRam {
    pub const SIZE: usize = 0x20;
}

impl Default for PRam {
    fn default() -> Self {
        Self([0xff; Self::SIZE])
    }
}

impl PpuDevice for PRam {
    fn read_ppu(&self, addr: u16) -> u8 {
        self.0[addr as usize % Self::SIZE]
    }

    fn write_ppu(&mut self, addr: u16, value: u8) {
        self.0[addr as usize % Self::SIZE] = value;
    }
}
