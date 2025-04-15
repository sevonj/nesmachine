use std::fmt::Debug;

use nesmc_emu::bus::Bus;
use nesmc_types::instruction::OpCode;

use crate::operand::Operand;

pub struct DisassInst {
    op_code: OpCode,
    operand: Operand,
}

impl DisassInst {
    pub fn from_read_bus(bus: &Bus, addr: u16) -> Self {
        let op_code = OpCode::from(bus.read(addr));
        let operand = Operand::from_read_bus(op_code, bus, addr);
        Self { op_code, operand }
    }

    pub const fn is_illegal(&self) -> bool {
        self.op_code.is_illegal()
    }
}

impl Debug for DisassInst {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let op_code = format!("{:02X?}", self.op_code);

        write!(f, "{op_code: <11} {:?}", self.operand)
    }
}
