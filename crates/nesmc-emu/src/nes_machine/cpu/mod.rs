mod status;

use status::CpuStatus;

#[derive(Debug)]
pub struct Cpu {
    pub a: u8,
    pub x: u8,
    pub y: u8,
    pub pc: u16,
    pub s: u8,
    pub p: CpuStatus,
}

impl Default for Cpu {
    fn default() -> Self {
        Self {
            a: 0,
            x: 0,
            y: 0,
            pc: 0xfffc,
            s: 0xfd,
            p: CpuStatus::default(),
        }
    }
}

impl Cpu {
    /// Reset button behavior
    pub fn reset(&mut self) {
        self.pc = 0xfffc;
        self.s = self.s.wrapping_sub(3);
        self.p.reset();
    }
}
