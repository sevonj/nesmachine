use crate::{bus::Bus, nes_machine::cpu::Cpu};

impl Cpu {
    fn branch(&mut self, value: u8) {
        let signed = value as i8;
        self.pc = self.pc.wrapping_add_signed(signed as i16);
    }

    /// Branch if carry clear
    pub(super) fn instr_bcc_rel(&mut self, bus: &mut Bus) {
        if !self.p.c {
            let value = self.fetch_operand_rel(bus);
            self.branch(value);
        }
    }
    /// Branch if carry set
    pub(super) fn instr_bcs_rel(&mut self, bus: &mut Bus) {
        if self.p.c {
            let value = self.fetch_operand_rel(bus);
            self.branch(value);
        }
    }
    /// Branch if equal
    pub(super) fn instr_beq_rel(&mut self, bus: &mut Bus) {
        if self.p.z {
            let value = self.fetch_operand_rel(bus);
            self.branch(value);
        }
    }
    /// Branch not equal
    pub(super) fn instr_bne_rel(&mut self, bus: &mut Bus) {
        if !self.p.z {
            let value = self.fetch_operand_rel(bus);
            self.branch(value);
        }
    }
    /// Branch if plus (not negative)
    pub(super) fn instr_bpl_rel(&mut self, bus: &mut Bus) {
        if !self.p.n {
            let value = self.fetch_operand_rel(bus);
            self.branch(value);
        }
    }
    /// Branch if minus (negative)
    pub(super) fn instr_bmi_rel(&mut self, bus: &mut Bus) {
        if self.p.n {
            let value = self.fetch_operand_rel(bus);
            self.branch(value);
        }
    }
    /// Branch if overflow clear
    pub(super) fn instr_bvc_rel(&mut self, bus: &mut Bus) {
        if !self.p.v {
            let value = self.fetch_operand_rel(bus);
            self.branch(value);
        }
    }
    /// Branch if overflow set
    pub(super) fn instr_bvs_rel(&mut self, bus: &mut Bus) {
        if self.p.v {
            let value = self.fetch_operand_rel(bus);
            self.branch(value);
        }
    }
}
