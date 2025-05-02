use super::CpuDevice;

/// CPU Internal RAM
#[derive(Debug)]
pub struct IRam([u8; Self::SIZE]);

impl IRam {
    pub const SIZE: usize = 0x800;
}

impl Default for IRam {
    fn default() -> Self {
        /*
        Kinda meaningless in practice, but uninitialized memory tends to be
        mostly 0xFF. https://forums.nesdev.org/viewtopic.php?t=13245
        */
        Self([0xff; Self::SIZE])
    }
}

impl CpuDevice for IRam {
    fn read(&mut self, addr: u16) -> u8 {
        self.0[addr as usize % Self::SIZE]
    }

    fn read_immutable(&self, addr: u16) -> u8 {
        self.0[addr as usize % Self::SIZE]
    }

    fn write(&mut self, addr: u16, value: u8) {
        self.0[addr as usize % Self::SIZE] = value;
    }
}
