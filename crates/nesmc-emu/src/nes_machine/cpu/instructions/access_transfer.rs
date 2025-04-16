use crate::{bus::Bus, nes_machine::cpu::Cpu};

impl Cpu {
    fn instr_lda(&mut self, op: u8) {
        self.a = op;
        self.set_zero(op);
        self.set_negative(op);
    }

    fn instr_ldx(&mut self, op: u8) {
        self.x = op;
        self.set_zero(op);
        self.set_negative(op);
    }

    fn instr_ldy(&mut self, op: u8) {
        self.y = op;
        self.set_zero(op);
        self.set_negative(op);
    }

    pub(super) fn instr_lda_abs(&mut self, bus: &mut Bus) {
        let op = self.fetch_operand_abs(bus);
        self.instr_lda(op);
    }
    pub(super) fn instr_lda_absx(&mut self, bus: &mut Bus) {
        let op = self.fetch_operand_absx(bus);
        self.instr_lda(op);
    }
    pub(super) fn instr_lda_absy(&mut self, bus: &mut Bus) {
        let op = self.fetch_operand_absy(bus);
        self.instr_lda(op);
    }
    pub(super) fn instr_lda_imm(&mut self, bus: &mut Bus) {
        let op = self.fetch_operand_imm(bus);
        self.instr_lda(op);
    }
    pub(super) fn instr_lda_xind(&mut self, bus: &mut Bus) {
        let op = self.fetch_operand_xind(bus);
        self.instr_lda(op);
    }
    pub(super) fn instr_lda_indy(&mut self, bus: &mut Bus) {
        let op = self.fetch_operand_indy(bus);
        self.instr_lda(op);
    }
    pub(super) fn instr_lda_zpg(&mut self, bus: &mut Bus) {
        let op = self.fetch_operand_zpg(bus);
        self.instr_lda(op);
    }
    pub(super) fn instr_lda_zpgx(&mut self, bus: &mut Bus) {
        let op = self.fetch_operand_zpgx(bus);
        self.instr_lda(op);
    }

    pub(super) fn instr_ldx_abs(&mut self, bus: &mut Bus) {
        let op = self.fetch_operand_abs(bus);
        self.instr_ldx(op);
    }
    pub(super) fn instr_ldx_absy(&mut self, bus: &mut Bus) {
        let op = self.fetch_operand_absy(bus);
        self.instr_ldx(op);
    }
    pub(super) fn instr_ldx_imm(&mut self, bus: &mut Bus) {
        let op = self.fetch_operand_imm(bus);
        self.instr_ldx(op);
    }
    pub(super) fn instr_ldx_zpg(&mut self, bus: &mut Bus) {
        let op = self.fetch_operand_zpg(bus);
        self.instr_ldx(op);
    }
    pub(super) fn instr_ldx_zpgy(&mut self, bus: &mut Bus) {
        let op = self.fetch_operand_zpgx(bus);
        self.instr_ldx(op);
    }

    pub(super) fn instr_ldy_abs(&mut self, bus: &mut Bus) {
        let op = self.fetch_operand_abs(bus);
        self.instr_ldy(op);
    }
    pub(super) fn instr_ldy_absx(&mut self, bus: &mut Bus) {
        let op = self.fetch_operand_absx(bus);
        self.instr_ldy(op);
    }
    pub(super) fn instr_ldy_imm(&mut self, bus: &mut Bus) {
        let op = self.fetch_operand_imm(bus);
        self.instr_ldy(op);
    }
    pub(super) fn instr_ldy_zpg(&mut self, bus: &mut Bus) {
        let op = self.fetch_operand_zpg(bus);
        self.instr_ldy(op);
    }
    pub(super) fn instr_ldy_zpgx(&mut self, bus: &mut Bus) {
        let op = self.fetch_operand_zpgx(bus);
        self.instr_ldy(op);
    }

    pub(super) fn instr_sta_abs(&mut self, bus: &mut Bus) {
        let addr = self.fetch_address_abs(bus);
        bus.write(addr, self.a);
    }
    pub(super) fn instr_sta_absx(&mut self, bus: &mut Bus) {
        let addr = self.fetch_address_absx(bus);
        bus.write(addr, self.a);
    }
    pub(super) fn instr_sta_absy(&mut self, bus: &mut Bus) {
        let addr = self.fetch_address_absy(bus);
        bus.write(addr, self.a);
    }
    pub(super) fn instr_sta_xind(&mut self, bus: &mut Bus) {
        let addr = self.fetch_address_xind(bus);
        bus.write(addr, self.a);
    }
    pub(super) fn instr_sta_indy(&mut self, bus: &mut Bus) {
        let addr = self.fetch_address_indy(bus);
        bus.write(addr, self.a);
    }
    pub(super) fn instr_sta_zpg(&mut self, bus: &mut Bus) {
        let addr = self.fetch_address_zpg(bus);
        bus.write(addr, self.a);
    }
    pub(super) fn instr_sta_zpgx(&mut self, bus: &mut Bus) {
        let addr = self.fetch_address_zpgx(bus);
        bus.write(addr, self.a);
    }

    pub(super) fn instr_stx_abs(&mut self, bus: &mut Bus) {
        let addr = self.fetch_address_abs(bus);
        bus.write(addr, self.x);
    }
    pub(super) fn instr_stx_zpg(&mut self, bus: &mut Bus) {
        let addr = self.fetch_address_zpg(bus);
        bus.write(addr, self.x);
    }
    pub(super) fn instr_stx_zpgy(&mut self, bus: &mut Bus) {
        let addr = self.fetch_address_zpgy(bus);
        bus.write(addr, self.x);
    }

    pub(super) fn instr_sty_abs(&mut self, bus: &mut Bus) {
        let addr = self.fetch_address_abs(bus);
        bus.write(addr, self.y);
    }
    pub(super) fn instr_sty_zpg(&mut self, bus: &mut Bus) {
        let addr = self.fetch_address_zpg(bus);
        bus.write(addr, self.y);
    }
    pub(super) fn instr_sty_zpgx(&mut self, bus: &mut Bus) {
        let addr = self.fetch_address_zpgx(bus);
        bus.write(addr, self.y);
    }

    pub(super) fn instr_tax_impl(&mut self) {
        self.x = self.a;
        self.set_zero(self.x);
        self.set_negative(self.x);
    }

    pub(super) fn instr_tay_impl(&mut self) {
        self.y = self.a;
        self.set_zero(self.y);
        self.set_negative(self.y);
    }

    pub(super) fn instr_txa_impl(&mut self) {
        self.a = self.x;
        self.set_zero(self.a);
        self.set_negative(self.a);
    }

    pub(super) fn instr_tya_impl(&mut self) {
        self.a = self.y;
        self.set_zero(self.a);
        self.set_negative(self.a);
    }
}
