use crate::{bus::Bus, nes_machine::cpu::Cpu};

impl Cpu {
    fn instr_and(&mut self, value: u8) {
        self.a &= value;
        self.set_zero(self.a);
        self.set_negative(self.a);
    }

    fn instr_ora(&mut self, value: u8) {
        self.a |= value;
        self.set_zero(self.a);
        self.set_negative(self.a);
    }

    fn instr_eor(&mut self, value: u8) {
        self.a ^= value;
        self.set_zero(self.a);
        self.set_negative(self.a);
    }

    fn instr_bit(&mut self, value: u8) {
        let result = self.a & value;
        self.set_zero(result);
        self.p.v = value & (1 << 6) != 0;
        self.p.n = value & (1 << 7) != 0;
    }

    pub(super) fn instr_and_abs(&mut self, bus: &mut Bus) {
        let value = self.fetch_operand_abs(bus);
        self.instr_and(value);
    }
    pub(super) fn instr_and_absx(&mut self, bus: &mut Bus) {
        let value = self.fetch_operand_absx(bus);
        self.instr_and(value);
    }
    pub(super) fn instr_and_absy(&mut self, bus: &mut Bus) {
        let value = self.fetch_operand_absy(bus);
        self.instr_and(value);
    }
    pub(super) fn instr_and_imm(&mut self, bus: &mut Bus) {
        let value = self.fetch_operand_imm(bus);
        self.instr_and(value);
    }
    pub(super) fn instr_and_xind(&mut self, bus: &mut Bus) {
        let value = self.fetch_operand_xind(bus);
        self.instr_and(value);
    }
    pub(super) fn instr_and_indy(&mut self, bus: &mut Bus) {
        let value = self.fetch_operand_indy(bus);
        self.instr_and(value);
    }
    pub(super) fn instr_and_zpg(&mut self, bus: &mut Bus) {
        let value = self.fetch_operand_zpg(bus);
        self.instr_and(value);
    }
    pub(super) fn instr_and_zpgx(&mut self, bus: &mut Bus) {
        let value = self.fetch_operand_zpgx(bus);
        self.instr_and(value);
    }

    pub(super) fn instr_ora_abs(&mut self, bus: &mut Bus) {
        let value = self.fetch_operand_abs(bus);
        self.instr_ora(value);
    }
    pub(super) fn instr_ora_absx(&mut self, bus: &mut Bus) {
        let value = self.fetch_operand_absx(bus);
        self.instr_ora(value);
    }
    pub(super) fn instr_ora_absy(&mut self, bus: &mut Bus) {
        let value = self.fetch_operand_absy(bus);
        self.instr_ora(value);
    }
    pub(super) fn instr_ora_imm(&mut self, bus: &mut Bus) {
        let value = self.fetch_operand_imm(bus);
        self.instr_ora(value);
    }
    pub(super) fn instr_ora_xind(&mut self, bus: &mut Bus) {
        let value = self.fetch_operand_xind(bus);
        self.instr_ora(value);
    }
    pub(super) fn instr_ora_indy(&mut self, bus: &mut Bus) {
        let value = self.fetch_operand_indy(bus);
        self.instr_ora(value);
    }
    pub(super) fn instr_ora_zpg(&mut self, bus: &mut Bus) {
        let value = self.fetch_operand_zpg(bus);
        self.instr_ora(value);
    }
    pub(super) fn instr_ora_zpgx(&mut self, bus: &mut Bus) {
        let value = self.fetch_operand_zpgx(bus);
        self.instr_ora(value);
    }

    pub(super) fn instr_eor_abs(&mut self, bus: &mut Bus) {
        let value = self.fetch_operand_abs(bus);
        self.instr_eor(value);
    }
    pub(super) fn instr_eor_absx(&mut self, bus: &mut Bus) {
        let value = self.fetch_operand_absx(bus);
        self.instr_eor(value);
    }
    pub(super) fn instr_eor_absy(&mut self, bus: &mut Bus) {
        let value = self.fetch_operand_absy(bus);
        self.instr_eor(value);
    }
    pub(super) fn instr_eor_imm(&mut self, bus: &mut Bus) {
        let value = self.fetch_operand_imm(bus);
        self.instr_eor(value);
    }
    pub(super) fn instr_eor_xind(&mut self, bus: &mut Bus) {
        let value = self.fetch_operand_xind(bus);
        self.instr_eor(value);
    }
    pub(super) fn instr_eor_indy(&mut self, bus: &mut Bus) {
        let value = self.fetch_operand_indy(bus);
        self.instr_eor(value);
    }
    pub(super) fn instr_eor_zpg(&mut self, bus: &mut Bus) {
        let value = self.fetch_operand_zpg(bus);
        self.instr_eor(value);
    }
    pub(super) fn instr_eor_zpgx(&mut self, bus: &mut Bus) {
        let value = self.fetch_operand_zpgx(bus);
        self.instr_eor(value);
    }

    pub(super) fn instr_bit_abs(&mut self, bus: &mut Bus) {
        let value = self.fetch_operand_abs(bus);
        self.instr_bit(value);
    }
    pub(super) fn instr_bit_zpg(&mut self, bus: &mut Bus) {
        let value = self.fetch_operand_zpg(bus);
        self.instr_bit(value);
    }
}
