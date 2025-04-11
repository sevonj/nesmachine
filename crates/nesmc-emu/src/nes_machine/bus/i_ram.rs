use super::Device;

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

impl Device for IRam {
    fn read(&self, addr: usize) -> u8 {
        self.0[addr % Self::SIZE]
    }

    fn write(&mut self, addr: usize, value: u8) {
        self.0[addr % Self::SIZE] = value;
    }
}
