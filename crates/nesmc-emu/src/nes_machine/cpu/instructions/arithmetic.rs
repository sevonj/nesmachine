use crate::{bus::Bus, nes_machine::cpu::Cpu};

impl Cpu {
    fn instr_adc(&mut self, op: u8) {
        let carry = if self.p.c { 1 } else { 0 };
        let (result, ov1) = self.a.overflowing_add(op); // add data
        let (result, ov2) = result.overflowing_add(carry); // add carry from previous operation
        self.set_carry(ov1 || ov2);
        self.set_zero(result);
        self.set_negative(result);
        self.set_overflow(op, result);
        self.a = result;
    }

    fn instr_sbc(&mut self, op: u8) {
        let carry = if self.p.c { 1 } else { 0 };
        let (result, _ov1) = self.a.overflowing_add(!op); // add data
        let (result, _ov2) = result.overflowing_add(carry); // add carry from previous operation
        self.set_carry(result <= self.a);
        self.set_zero(result);
        self.set_negative(result);
        self.set_overflow(!op, result);
        self.a = result;
    }

    fn instr_inc(&mut self, bus: &mut Bus, addr: u16) {
        let value = bus.read(addr).wrapping_add(1);
        self.set_zero(value);
        self.set_negative(value);
        bus.write(addr, value);
    }

    fn instr_dec(&mut self, bus: &mut Bus, addr: u16) {
        let value = bus.read(addr).wrapping_sub(1);
        self.set_zero(value);
        self.set_negative(value);
        bus.write(addr, value);
    }

    pub(super) fn instr_adc_abs(&mut self, bus: &mut Bus) {
        let op = self.fetch_operand_abs(bus);
        self.instr_adc(op);
    }
    pub(super) fn instr_adc_absx(&mut self, bus: &mut Bus) {
        let op = self.fetch_operand_absx(bus);
        self.instr_adc(op);
    }
    pub(super) fn instr_adc_absy(&mut self, bus: &mut Bus) {
        let op = self.fetch_operand_absy(bus);
        self.instr_adc(op);
    }
    pub(super) fn instr_adc_imm(&mut self, bus: &mut Bus) {
        let op = self.fetch_operand_imm(bus);
        self.instr_adc(op);
    }
    pub(super) fn instr_adc_xind(&mut self, bus: &mut Bus) {
        let op = self.fetch_operand_xind(bus);
        self.instr_adc(op);
    }
    pub(super) fn instr_adc_indy(&mut self, bus: &mut Bus) {
        let op = self.fetch_operand_indy(bus);
        self.instr_adc(op);
    }
    pub(super) fn instr_adc_zpg(&mut self, bus: &mut Bus) {
        let op = self.fetch_operand_zpg(bus);
        self.instr_adc(op);
    }
    pub(super) fn instr_adc_zpgx(&mut self, bus: &mut Bus) {
        let op = self.fetch_operand_zpgx(bus);
        self.instr_adc(op);
    }

    pub(super) fn instr_sbc_abs(&mut self, bus: &mut Bus) {
        let op = self.fetch_operand_abs(bus);
        self.instr_sbc(op);
    }
    pub(super) fn instr_sbc_absx(&mut self, bus: &mut Bus) {
        let op = self.fetch_operand_absx(bus);
        self.instr_sbc(op);
    }
    pub(super) fn instr_sbc_absy(&mut self, bus: &mut Bus) {
        let op = self.fetch_operand_absy(bus);
        self.instr_sbc(op);
    }
    pub(super) fn instr_sbc_imm(&mut self, bus: &mut Bus) {
        let op = self.fetch_operand_imm(bus);
        self.instr_sbc(op);
    }
    pub(super) fn instr_sbc_xind(&mut self, bus: &mut Bus) {
        let op = self.fetch_operand_xind(bus);
        self.instr_sbc(op);
    }
    pub(super) fn instr_sbc_indy(&mut self, bus: &mut Bus) {
        let op = self.fetch_operand_indy(bus);
        self.instr_sbc(op);
    }
    pub(super) fn instr_sbc_zpg(&mut self, bus: &mut Bus) {
        let op = self.fetch_operand_zpg(bus);
        self.instr_sbc(op);
    }
    pub(super) fn instr_sbc_zpgx(&mut self, bus: &mut Bus) {
        let op = self.fetch_operand_zpgx(bus);
        self.instr_sbc(op);
    }

    pub(super) fn instr_inc_abs(&mut self, bus: &mut Bus) {
        let addr = self.fetch_address_abs(bus);
        self.instr_inc(bus, addr);
    }
    pub(super) fn instr_inc_absx(&mut self, bus: &mut Bus) {
        let addr = self.fetch_address_absx(bus);
        self.instr_inc(bus, addr);
    }
    pub(super) fn instr_inc_zpg(&mut self, bus: &mut Bus) {
        let addr = self.fetch_address_zpg(bus);
        self.instr_inc(bus, addr);
    }
    pub(super) fn instr_inc_zpgx(&mut self, bus: &mut Bus) {
        let addr = self.fetch_address_zpgx(bus);
        self.instr_inc(bus, addr);
    }

    pub(super) fn instr_dec_abs(&mut self, bus: &mut Bus) {
        let addr = self.fetch_address_abs(bus);
        self.instr_dec(bus, addr);
    }
    pub(super) fn instr_dec_absx(&mut self, bus: &mut Bus) {
        let addr = self.fetch_address_absx(bus);
        self.instr_dec(bus, addr);
    }
    pub(super) fn instr_dec_zpg(&mut self, bus: &mut Bus) {
        let addr = self.fetch_address_zpg(bus);
        self.instr_dec(bus, addr);
    }
    pub(super) fn instr_dec_zpgx(&mut self, bus: &mut Bus) {
        let addr = self.fetch_address_zpgx(bus);
        self.instr_dec(bus, addr);
    }

    pub(super) fn instr_inx_impl(&mut self) {
        self.x = self.x.wrapping_add(1);
        self.set_zero(self.x);
        self.set_negative(self.x);
    }

    pub(super) fn instr_dex_impl(&mut self) {
        self.x = self.x.wrapping_sub(1);
        self.set_zero(self.x);
        self.set_negative(self.x);
    }

    pub(super) fn instr_iny_impl(&mut self) {
        self.y = self.y.wrapping_add(1);
        self.set_zero(self.y);
        self.set_negative(self.y);
    }

    pub(super) fn instr_dey_impl(&mut self) {
        self.y = self.y.wrapping_sub(1);
        self.set_zero(self.y);
        self.set_negative(self.y);
    }
}
