use crate::{bus::Bus, nes_machine::cpu::Cpu};

impl Cpu {
    fn instr_lax(&mut self, value: u8) {
        self.set_negative(value);
        self.set_zero(value);
        self.a = value;
        self.x = value;
    }

    pub(super) fn instr_nop_imm(&mut self, bus: &mut Bus) {
        let _ = self.fetch_operand_imm(bus);
    }

    pub(super) fn instr_nop_abs(&mut self, bus: &mut Bus) {
        let _ = self.fetch_operand_abs(bus);
    }

    pub(super) fn instr_nop_absx(&mut self, bus: &mut Bus) {
        let _ = self.fetch_operand_absx(bus);
    }

    pub(super) fn instr_nop_zpg(&mut self, bus: &mut Bus) {
        let _ = self.fetch_operand_zpg(bus);
    }

    pub(super) fn instr_nop_zpgx(&mut self, bus: &mut Bus) {
        let _ = self.fetch_operand_zpgx(bus);
    }

    pub(super) fn instr_lax_abs(&mut self, bus: &mut Bus) {
        let value = self.fetch_operand_abs(bus);
        self.instr_lax(value);
    }
    pub(super) fn instr_lax_absy(&mut self, bus: &mut Bus) {
        let value = self.fetch_operand_absy(bus);
        self.instr_lax(value);
    }
    pub(super) fn instr_lax_xind(&mut self, bus: &mut Bus) {
        let value = self.fetch_operand_xind(bus);
        self.instr_lax(value);
    }
    pub(super) fn instr_lax_indy(&mut self, bus: &mut Bus) {
        let value = self.fetch_operand_indy(bus);
        self.instr_lax(value);
    }
    pub(super) fn instr_lax_zpg(&mut self, bus: &mut Bus) {
        let value = self.fetch_operand_zpg(bus);
        self.instr_lax(value);
    }
    pub(super) fn instr_lax_zpgy(&mut self, bus: &mut Bus) {
        let value = self.fetch_operand_zpgy(bus);
        self.instr_lax(value);
    }

    pub(super) fn instr_sax_abs(&mut self, bus: &mut Bus) {
        let addr = self.fetch_address_abs(bus);
        bus.write(addr, self.a & self.x);
    }
    pub(super) fn instr_sax_xind(&mut self, bus: &mut Bus) {
        let addr = self.fetch_address_xind(bus);
        bus.write(addr, self.a & self.x);
    }
    pub(super) fn instr_sax_zpg(&mut self, bus: &mut Bus) {
        let addr = self.fetch_address_zpg(bus);
        bus.write(addr, self.a & self.x);
    }
    pub(super) fn instr_sax_zpgy(&mut self, bus: &mut Bus) {
        let addr = self.fetch_address_zpgy(bus);
        bus.write(addr, self.a & self.x);
    }

    fn instr_dcp(&mut self, bus: &mut Bus, addr: u16) {
        let value = bus.read(addr).wrapping_sub(1);
        bus.write(addr, value);
        self.status.c = self.a >= value;
        self.status.z = self.a == value;
        self.set_negative(self.a.wrapping_sub(value));
    }
    pub(super) fn instr_dcp_abs(&mut self, bus: &mut Bus) {
        let addr = self.fetch_address_abs(bus);
        self.instr_dcp(bus, addr);
    }
    pub(super) fn instr_dcp_absx(&mut self, bus: &mut Bus) {
        let addr = self.fetch_address_absx(bus);
        self.instr_dcp(bus, addr);
    }
    pub(super) fn instr_dcp_absy(&mut self, bus: &mut Bus) {
        let addr = self.fetch_address_absy(bus);
        self.instr_dcp(bus, addr);
    }
    pub(super) fn instr_dcp_xind(&mut self, bus: &mut Bus) {
        let addr = self.fetch_address_xind(bus);
        self.instr_dcp(bus, addr);
    }
    pub(super) fn instr_dcp_indy(&mut self, bus: &mut Bus) {
        let addr = self.fetch_address_indy(bus);
        self.instr_dcp(bus, addr);
    }
    pub(super) fn instr_dcp_zpg(&mut self, bus: &mut Bus) {
        let addr = self.fetch_address_zpg(bus);
        self.instr_dcp(bus, addr);
    }
    pub(super) fn instr_dcp_zpgx(&mut self, bus: &mut Bus) {
        let addr = self.fetch_address_zpgx(bus);
        self.instr_dcp(bus, addr);
    }
}
