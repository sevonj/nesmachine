use crate::{
    bus::Bus,
    nes_machine::cpu::{Cpu, CpuStatus},
};

impl Cpu {
    fn instr_lda(&mut self, value: u8) {
        self.a = value;
        self.set_zero(value);
        self.set_negative(value);
    }

    fn instr_ldx(&mut self, value: u8) {
        self.x = value;
        self.set_zero(value);
        self.set_negative(value);
    }

    fn instr_ldy(&mut self, value: u8) {
        self.y = value;
        self.set_zero(value);
        self.set_negative(value);
    }

    pub(super) fn instr_lda_abs(&mut self, bus: &mut Bus) {
        let value = self.fetch_operand_abs(bus);
        self.instr_lda(value);
    }
    pub(super) fn instr_lda_absx(&mut self, bus: &mut Bus) {
        let value = self.fetch_operand_absx(bus);
        self.instr_lda(value);
    }
    pub(super) fn instr_lda_absy(&mut self, bus: &mut Bus) {
        let value = self.fetch_operand_absy(bus);
        self.instr_lda(value);
    }
    pub(super) fn instr_lda_imm(&mut self, bus: &mut Bus) {
        let value = self.fetch_operand_imm(bus);
        self.instr_lda(value);
    }
    pub(super) fn instr_lda_xind(&mut self, bus: &mut Bus) {
        let value = self.fetch_operand_xind(bus);
        self.instr_lda(value);
    }
    pub(super) fn instr_lda_indy(&mut self, bus: &mut Bus) {
        let value = self.fetch_operand_indy(bus);
        self.instr_lda(value);
    }
    pub(super) fn instr_lda_zpg(&mut self, bus: &mut Bus) {
        let value = self.fetch_operand_zpg(bus);
        self.instr_lda(value);
    }
    pub(super) fn instr_lda_zpgx(&mut self, bus: &mut Bus) {
        let value = self.fetch_operand_zpgx(bus);
        self.instr_lda(value);
    }

    pub(super) fn instr_ldx_abs(&mut self, bus: &mut Bus) {
        let value = self.fetch_operand_abs(bus);
        self.instr_ldx(value);
    }
    pub(super) fn instr_ldx_absy(&mut self, bus: &mut Bus) {
        let value = self.fetch_operand_absy(bus);
        self.instr_ldx(value);
    }
    pub(super) fn instr_ldx_imm(&mut self, bus: &mut Bus) {
        let value = self.fetch_operand_imm(bus);
        self.instr_ldx(value);
    }
    pub(super) fn instr_ldx_zpg(&mut self, bus: &mut Bus) {
        let value = self.fetch_operand_zpg(bus);
        self.instr_ldx(value);
    }
    pub(super) fn instr_ldx_zpgy(&mut self, bus: &mut Bus) {
        let value = self.fetch_operand_zpgy(bus);
        self.instr_ldx(value);
    }

    pub(super) fn instr_ldy_abs(&mut self, bus: &mut Bus) {
        let value = self.fetch_operand_abs(bus);
        self.instr_ldy(value);
    }
    pub(super) fn instr_ldy_absx(&mut self, bus: &mut Bus) {
        let value = self.fetch_operand_absx(bus);
        self.instr_ldy(value);
    }
    pub(super) fn instr_ldy_imm(&mut self, bus: &mut Bus) {
        let value = self.fetch_operand_imm(bus);
        self.instr_ldy(value);
    }
    pub(super) fn instr_ldy_zpg(&mut self, bus: &mut Bus) {
        let value = self.fetch_operand_zpg(bus);
        self.instr_ldy(value);
    }
    pub(super) fn instr_ldy_zpgx(&mut self, bus: &mut Bus) {
        let value = self.fetch_operand_zpgx(bus);
        self.instr_ldy(value);
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

    pub(super) fn instr_tsx_impl(&mut self) {
        self.x = self.sp;
        self.set_zero(self.x);
        self.set_negative(self.x);
    }

    pub(super) fn instr_txa_impl(&mut self) {
        self.a = self.x;
        self.set_zero(self.a);
        self.set_negative(self.a);
    }

    pub(super) fn instr_txs_impl(&mut self) {
        self.sp = self.x;
    }

    pub(super) fn instr_tya_impl(&mut self) {
        self.a = self.y;
        self.set_zero(self.a);
        self.set_negative(self.a);
    }

    /// Push A to stack
    pub(super) fn instr_pha_impl(&mut self, bus: &mut Bus) {
        self.push_stack(self.a, bus);
    }

    /// Pull A from stack
    pub(super) fn instr_pla_impl(&mut self, bus: &mut Bus) {
        self.a = self.pop_stack(bus);
        self.set_zero(self.a);
        self.set_negative(self.a);
    }

    /// Push status to stack
    pub(super) fn instr_php_impl(&mut self, bus: &mut Bus) {
        const BRK_FLAG: u8 = 0x10;
        let value = u8::from(self.status) | BRK_FLAG;
        self.push_stack(value, bus);
    }

    /// Pull status from stack
    pub(super) fn instr_plp_impl(&mut self, bus: &mut Bus) {
        self.status = CpuStatus::from(self.pop_stack(bus));
        // TODO: I flag should be delayed by 1 inst
    }
}
