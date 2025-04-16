use crate::{bus::Bus, nes_machine::cpu::Cpu};

impl Cpu {
    fn instr_asl(&mut self, bus: &mut Bus, addr: u16) {
        let value = bus.read(addr);
        let shifted = value << 1;
        self.set_carry(value & 0x80 != 0);
        self.set_zero(shifted);
        self.set_negative(shifted);
        bus.write(addr, shifted);
    }

    fn instr_lsr(&mut self, bus: &mut Bus, addr: u16) {
        let value = bus.read(addr);
        let shifted = value >> 1;
        self.set_carry(value & 0x01 != 0);
        self.set_zero(shifted);
        self.set_negative(shifted);
        bus.write(addr, shifted);
    }

    fn instr_rol(&mut self, bus: &mut Bus, addr: u16) {
        let value = bus.read(addr);
        let carry = if self.p.c { 1 } else { 0 };
        let shifted = (value << 1) | carry;
        self.set_carry(value & 0x80 != 0);
        self.set_zero(shifted);
        self.set_negative(shifted);
        bus.write(addr, shifted);
    }

    fn instr_ror(&mut self, bus: &mut Bus, addr: u16) {
        let value = bus.read(addr);
        let carry = if self.p.c { 1 } else { 0 };
        let shifted = (value >> 1) | carry;
        self.set_carry(value & 0x01 != 0);
        self.set_zero(shifted);
        self.set_negative(shifted);
        bus.write(addr, shifted);
    }

    pub(super) fn instr_asl_a(&mut self) {
        let shifted = self.a << 1;
        self.set_carry(self.a & 0x80 != 0);
        self.set_zero(shifted);
        self.set_negative(shifted);
        self.a = shifted;
    }
    pub(super) fn instr_asl_abs(&mut self, bus: &mut Bus) {
        let addr = self.fetch_address_abs(bus);
        self.instr_asl(bus, addr);
    }
    pub(super) fn instr_asl_absx(&mut self, bus: &mut Bus) {
        let addr = self.fetch_address_absx(bus);
        self.instr_asl(bus, addr);
    }
    pub(super) fn instr_asl_zpg(&mut self, bus: &mut Bus) {
        let addr = self.fetch_address_zpg(bus);
        self.instr_asl(bus, addr);
    }
    pub(super) fn instr_asl_zpgx(&mut self, bus: &mut Bus) {
        let addr = self.fetch_address_zpgx(bus);
        self.instr_asl(bus, addr);
    }

    pub(super) fn instr_lsr_a(&mut self) {
        let shifted = self.a >> 1;
        self.set_carry(self.a & 0x01 != 0);
        self.set_zero(shifted);
        self.set_negative(shifted);
        self.a = shifted;
    }
    pub(super) fn instr_lsr_abs(&mut self, bus: &mut Bus) {
        let addr = self.fetch_address_abs(bus);
        self.instr_lsr(bus, addr);
    }
    pub(super) fn instr_lsr_absx(&mut self, bus: &mut Bus) {
        let addr = self.fetch_address_absx(bus);
        self.instr_lsr(bus, addr);
    }
    pub(super) fn instr_lsr_zpg(&mut self, bus: &mut Bus) {
        let addr = self.fetch_address_zpg(bus);
        self.instr_lsr(bus, addr);
    }
    pub(super) fn instr_lsr_zpgx(&mut self, bus: &mut Bus) {
        let addr = self.fetch_address_zpgx(bus);
        self.instr_lsr(bus, addr);
    }

    pub(super) fn instr_rol_a(&mut self) {
        let carry = if self.p.c { 1 } else { 0 };
        let shifted = (self.a << 1) | carry;
        self.set_carry(self.a & 0x80 != 0);
        self.set_zero(shifted);
        self.set_negative(shifted);
        self.a = shifted;
    }
    pub(super) fn instr_rol_abs(&mut self, bus: &mut Bus) {
        let addr = self.fetch_address_abs(bus);
        self.instr_rol(bus, addr);
    }
    pub(super) fn instr_rol_absx(&mut self, bus: &mut Bus) {
        let addr = self.fetch_address_absx(bus);
        self.instr_rol(bus, addr);
    }
    pub(super) fn instr_rol_zpg(&mut self, bus: &mut Bus) {
        let addr = self.fetch_address_zpg(bus);
        self.instr_rol(bus, addr);
    }
    pub(super) fn instr_rol_zpgx(&mut self, bus: &mut Bus) {
        let addr = self.fetch_address_zpgx(bus);
        self.instr_rol(bus, addr);
    }

    pub(super) fn instr_ror_a(&mut self) {
        let carry = if self.p.c { 1 } else { 0 };
        let shifted = (self.a >> 1) | carry;
        self.set_carry(self.a & 0x01 != 0);
        self.set_zero(shifted);
        self.set_negative(shifted);
        self.a = shifted;
    }
    pub(super) fn instr_ror_abs(&mut self, bus: &mut Bus) {
        let addr = self.fetch_address_abs(bus);
        self.instr_ror(bus, addr);
    }
    pub(super) fn instr_ror_absx(&mut self, bus: &mut Bus) {
        let addr = self.fetch_address_absx(bus);
        self.instr_ror(bus, addr);
    }
    pub(super) fn instr_ror_zpg(&mut self, bus: &mut Bus) {
        let addr = self.fetch_address_zpg(bus);
        self.instr_ror(bus, addr);
    }
    pub(super) fn instr_ror_zpgx(&mut self, bus: &mut Bus) {
        let addr = self.fetch_address_zpgx(bus);
        self.instr_ror(bus, addr);
    }
}
