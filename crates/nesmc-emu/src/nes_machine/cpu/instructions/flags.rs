use crate::nes_machine::cpu::Cpu;

impl Cpu {
    /// Clear carry
    pub(super) fn instr_clc(&mut self) {
        self.status.c = false;
    }

    /// Clear decimal
    pub(super) fn instr_cld(&mut self) {
        self.status.d = false;
    }

    /// Clear interrupt disable
    pub(super) fn instr_cli(&mut self) {
        self.status.i = false;
    }

    /// Clear overflow
    pub(super) fn instr_clv(&mut self) {
        self.status.v = false;
    }

    /// Set carry
    pub(super) fn instr_sec(&mut self) {
        self.status.c = true;
    }

    /// Set decimal
    pub(super) fn instr_sed(&mut self) {
        self.status.d = true;
    }

    /// Set interrupt disable
    pub(super) fn instr_sei(&mut self) {
        self.status.i = true;
    }
}
