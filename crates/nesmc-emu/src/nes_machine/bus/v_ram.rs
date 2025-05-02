use super::PpuDevice;

/// PPU VRAM
#[derive(Debug)]
pub struct VRam([u8; Self::SIZE]);

impl VRam {
    pub const SIZE: usize = 0x2000;
}

impl Default for VRam {
    fn default() -> Self {
        Self([0xff; Self::SIZE])
    }
}

impl PpuDevice for VRam {
    fn read_ppu(&self, addr: u16) -> u8 {
        self.0[addr as usize % Self::SIZE]
    }

    fn write_ppu(&mut self, addr: u16, value: u8) {
        self.0[addr as usize % Self::SIZE] = value;
    }
}
