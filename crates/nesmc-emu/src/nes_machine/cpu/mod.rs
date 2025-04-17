mod flags;
mod instructions;
mod operand;
mod status;

use nesmc_types::instruction::OpCode;
pub use status::CpuStatus;

use super::bus::Bus;

#[derive(Debug, PartialEq, Eq)]
pub struct Cpu {
    pub a: u8,
    pub x: u8,
    pub y: u8,
    pub pc: u16,
    pub sp: u8,
    pub status: CpuStatus,
}

impl std::fmt::Display for Cpu {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "A:{:02X} x:{:02X} Y:{:02X} PC:{:04X} SP:{:02X} P:{}",
            self.a, self.x, self.y, self.pc, self.sp, self.status
        )
    }
}

impl Cpu {
    const INIT_VECTOR: u16 = 0xfffc;

    pub fn new(bus: &Bus) -> Self {
        Self {
            a: 0,
            x: 0,
            y: 0,
            pc: read_u16(bus, Self::INIT_VECTOR),
            sp: 0xfd,
            status: CpuStatus::default(),
        }
    }

    /// Reset button behavior
    pub fn reset(&mut self, bus: &Bus) {
        self.pc = read_u16(bus, Self::INIT_VECTOR);
        self.sp = self.sp.wrapping_sub(3);
        self.status.reset();
    }

    /// Step one CPU instruction
    /// Returns the number of CPU cycles spent.
    pub fn step(&mut self, bus: &mut Bus) -> usize {
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

    fn push_stack(&mut self, value: u8, bus: &mut Bus) {
        bus.write(0x0100 + self.sp as u16, value);
        self.sp = self.sp.wrapping_sub(1);
    }

    fn pop_stack(&mut self, bus: &mut Bus) -> u8 {
        self.sp = self.sp.wrapping_add(1);
        bus.read(0x0100 + self.sp as u16)
    }
}

fn read_u16(bus: &Bus, addr: u16) -> u16 {
    let lo_byte = bus.read(addr) as u16;
    let hi_byte = bus.read(addr.wrapping_add(1)) as u16;
    (hi_byte << 8) | lo_byte
}
