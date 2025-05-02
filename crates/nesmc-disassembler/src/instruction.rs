use std::fmt::Debug;

use nesmc_emu::NesMachine;
use nesmc_types::instruction::OpCode;

use crate::operand::Operand;

pub struct DisassInst {
    op_code: OpCode,
    operand: Operand,
}

impl DisassInst {
    pub fn from_read_machine(machine: &NesMachine, addr: u16) -> Self {
        let op_code = OpCode::from(machine.bus.read_immutable(addr));
        let operand = Operand::from_read_machine(op_code, machine, addr);
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
