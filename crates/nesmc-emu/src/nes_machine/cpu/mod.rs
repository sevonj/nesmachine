mod flags;
mod instructions;
mod operand;
mod status;

use nesmc_types::instruction::OpCode;
use status::CpuStatus;

use super::bus::Bus;

#[derive(Debug)]
pub struct Cpu {
    pub a: u8,
    pub x: u8,
    pub y: u8,
    pub pc: u16,
    pub sp: u8,
    pub status: CpuStatus,
}

impl Default for Cpu {
    fn default() -> Self {
        Self {
            a: 0,
            x: 0,
            y: 0,
            pc: 0xfffc,
            sp: 0xfd,
            status: CpuStatus::default(),
        }
    }
}

impl Cpu {
    /// Reset button behavior
    pub fn reset(&mut self) {
        self.pc = 0xfffc;
        self.sp = self.sp.wrapping_sub(3);
        self.status.reset();
    }

    /// Step one CPU instruction
    pub fn step(&mut self, bus: &mut Bus) {
        // vblank interrupt
        /*let ppu_status = bus.read(0x2002);
        if ppu_status & 0b1000_0000 != 0 {
            bus.write(0x2002, ppu_status & 0b0111_1111);
            self.nmi(bus);
            return;
        }*/
        // Fetch instruction
        let op_code = OpCode::from(bus.read(self.pc));
        self.inc_pc();
        self.exec_instruction(bus, op_code)
    }

    /// Increment PC convenience shortcut
    fn inc_pc(&mut self) {
        self.pc = self.pc.wrapping_add(1);
    }

    fn push_stack(&mut self, data: u8, bus: &mut Bus) {
        bus.write(0x0100 + self.sp as u16, data);
        self.sp = self.sp.wrapping_sub(1);
    }

    fn pop_stack(&mut self, bus: &mut Bus) -> u8 {
        self.sp = self.sp.wrapping_add(1);
        bus.read(0x0100 + self.sp as u16)
    }
}
