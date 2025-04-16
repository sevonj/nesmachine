use crate::bus::Bus;

use super::Cpu;

impl Cpu {
    pub(super) fn fetch_operand_imm(&mut self, bus: &mut Bus) -> u8 {
        let operand = bus.read(self.pc);
        self.inc_pc();
        operand
    }

    pub(super) fn fetch_operand_abs(&mut self, bus: &mut Bus) -> u8 {
        let address = read_u16(bus, self.pc);
        self.inc_pc();
        self.inc_pc();
        bus.read(address)
    }

    pub(super) fn fetch_operand_absx(&mut self, bus: &mut Bus) -> u8 {
        let address = read_u16(bus, self.pc).wrapping_add(self.x as u16);
        self.inc_pc();
        self.inc_pc();
        bus.read(address)
    }

    pub(super) fn fetch_operand_absy(&mut self, bus: &mut Bus) -> u8 {
        let address = read_u16(bus, self.pc).wrapping_add(self.y as u16);
        self.inc_pc();
        self.inc_pc();
        bus.read(address)
    }

    pub(super) fn fetch_operand_xind(&mut self, bus: &mut Bus) -> u8 {
        let address = bus.read(self.pc).wrapping_add(self.x) as u16;
        self.inc_pc();
        let indirect_address = read_u16(bus, address);
        bus.read(indirect_address)
    }

    pub(super) fn fetch_operand_indy(&mut self, bus: &mut Bus) -> u8 {
        let address = bus.read(self.pc);
        self.inc_pc();
        let indirect_address = read_u16(bus, address as u16);
        let address = indirect_address.wrapping_add(self.y as u16);
        bus.read(address)
    }

    pub(super) fn fetch_operand_rel(&mut self, bus: &mut Bus) -> u8 {
        let operand = bus.read(self.pc);
        self.inc_pc();
        operand
    }

    pub(super) fn fetch_operand_zpg(&mut self, bus: &mut Bus) -> u8 {
        let address = bus.read(self.pc) as u16;
        self.inc_pc();
        bus.read(address)
    }

    pub(super) fn fetch_operand_zpgx(&mut self, bus: &mut Bus) -> u8 {
        let address = bus.read(self.pc).wrapping_add(self.x) as u16;
        self.inc_pc();
        bus.read(address)
    }

    pub(crate) fn fetch_address_abs(&mut self, bus: &mut Bus) -> u16 {
        let address = read_u16(bus, self.pc);
        self.inc_pc();
        self.inc_pc();
        address
    }

    pub(crate) fn fetch_address_absx(&mut self, bus: &mut Bus) -> u16 {
        let address = read_u16(bus, self.pc).wrapping_add(self.x as u16);
        self.inc_pc();
        self.inc_pc();
        address
    }

    pub(crate) fn fetch_address_absy(&mut self, bus: &mut Bus) -> u16 {
        let address = read_u16(bus, self.pc).wrapping_add(self.y as u16);
        self.inc_pc();
        self.inc_pc();
        address
    }

    pub(crate) fn fetch_address_ind(&mut self, bus: &mut Bus) -> u16 {
        let address = read_u16(bus, self.pc);
        self.inc_pc();
        self.inc_pc();
        read_u16(bus, address)
    }

    pub(crate) fn fetch_address_xind(&mut self, bus: &mut Bus) -> u16 {
        let address = self.fetch_address_zpgx(bus);
        read_u16(bus, address)
    }

    pub(crate) fn fetch_address_indy(&mut self, bus: &mut Bus) -> u16 {
        let address = bus.read(self.pc);
        self.inc_pc();
        read_u16(bus, address as u16).wrapping_add(self.y as u16)
    }

    pub(crate) fn fetch_address_zpg(&mut self, bus: &mut Bus) -> u16 {
        let address = bus.read(self.pc) as u16;
        self.inc_pc();
        address
    }

    pub(crate) fn fetch_address_zpgx(&mut self, bus: &mut Bus) -> u16 {
        let address = bus.read(self.pc).wrapping_add(self.x) as u16;
        self.inc_pc();
        address
    }

    pub(crate) fn fetch_address_zpgy(&mut self, bus: &mut Bus) -> u16 {
        let address = bus.read(self.pc).wrapping_add(self.y) as u16;
        self.inc_pc();
        address
    }
}

fn read_u16(bus: &mut Bus, addr: u16) -> u16 {
    let lo_byte = bus.read(addr) as u16;
    let hi_byte = bus.read(addr.wrapping_add(1)) as u16;
    (hi_byte << 8) | lo_byte
}
