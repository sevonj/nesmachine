use crate::nes_machine::cpu::Cpu;

impl Cpu {
    /// Clear carry
    pub(super) fn instr_clc(&mut self) -> usize {
        self.status.c = false;
        2
    }

    /// Clear decimal
    pub(super) fn instr_cld(&mut self) -> usize {
        self.status.d = false;
        2
    }

    /// Clear interrupt disable
    pub(super) fn instr_cli(&mut self) -> usize {
        self.status.i = false;
        2
    }

    /// Clear overflow
    pub(super) fn instr_clv(&mut self) -> usize {
        self.status.v = false;
        2
    }

    /// Set carry
    pub(super) fn instr_sec(&mut self) -> usize {
        self.status.c = true;
        2
    }

    /// Set decimal
    pub(super) fn instr_sed(&mut self) -> usize {
        self.status.d = true;
        2
    }

    /// Set interrupt disable
    pub(super) fn instr_sei(&mut self) -> usize {
        self.status.i = true;
        2
    }
}
