use crate::{bus::Bus, nes_machine::cpu::Cpu};

impl Cpu {
    fn instr_cmp(&mut self, value: u8) {
        let result = self.a.wrapping_sub(value);
        self.status.c = self.a >= value;
        self.set_zero(result);
        self.set_negative(result);
    }

    fn instr_cpx(&mut self, value: u8) {
        let result = self.x.wrapping_sub(value);
        self.status.c = self.x >= value;
        self.set_zero(result);
        self.set_negative(result);
    }

    fn instr_cpy(&mut self, value: u8) {
        let result = self.y.wrapping_sub(value);
        self.status.c = self.y >= value;
        self.set_zero(result);
        self.set_negative(result);
    }

    pub(super) fn instr_cmp_abs(&mut self, bus: &mut Bus) {
        let addr = self.fetch_operand_abs(bus);
        self.instr_cmp(addr);
    }
    pub(super) fn instr_cmp_absx(&mut self, bus: &mut Bus) {
        let addr = self.fetch_operand_absx(bus);
        self.instr_cmp(addr);
    }
    pub(super) fn instr_cmp_absy(&mut self, bus: &mut Bus) {
        let addr = self.fetch_operand_absy(bus);
        self.instr_cmp(addr);
    }
    pub(super) fn instr_cmp_imm(&mut self, bus: &mut Bus) {
        let addr = self.fetch_operand_imm(bus);
        self.instr_cmp(addr);
    }
    pub(super) fn instr_cmp_xind(&mut self, bus: &mut Bus) {
        let addr = self.fetch_operand_xind(bus);
        self.instr_cmp(addr);
    }
    pub(super) fn instr_cmp_indy(&mut self, bus: &mut Bus) {
        let addr = self.fetch_operand_indy(bus);
        self.instr_cmp(addr);
    }
    pub(super) fn instr_cmp_zpg(&mut self, bus: &mut Bus) {
        let addr = self.fetch_operand_zpg(bus);
        self.instr_cmp(addr);
    }
    pub(super) fn instr_cmp_zpgx(&mut self, bus: &mut Bus) {
        let addr = self.fetch_operand_zpgx(bus);
        self.instr_cmp(addr);
    }

    pub(super) fn instr_cpx_abs(&mut self, bus: &mut Bus) {
        let addr = self.fetch_operand_abs(bus);
        self.instr_cpx(addr);
    }
    pub(super) fn instr_cpx_imm(&mut self, bus: &mut Bus) {
        let addr = self.fetch_operand_imm(bus);
        self.instr_cpx(addr);
    }
    pub(super) fn instr_cpx_zpg(&mut self, bus: &mut Bus) {
        let addr = self.fetch_operand_zpg(bus);
        self.instr_cpx(addr);
    }

    pub(super) fn instr_cpy_abs(&mut self, bus: &mut Bus) {
        let addr = self.fetch_operand_abs(bus);
        self.instr_cpy(addr);
    }
    pub(super) fn instr_cpy_imm(&mut self, bus: &mut Bus) {
        let addr = self.fetch_operand_imm(bus);
        self.instr_cpy(addr);
    }
    pub(super) fn instr_cpy_zpg(&mut self, bus: &mut Bus) {
        let addr = self.fetch_operand_zpg(bus);
        self.instr_cpy(addr);
    }
}
