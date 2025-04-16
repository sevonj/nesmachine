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
        let address = self.fetch_address_xind(bus);
        bus.read(address)
    }

    pub(super) fn fetch_operand_indy(&mut self, bus: &mut Bus) -> u8 {
        let address = self.fetch_address_indy(bus);
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

    pub(super) fn fetch_operand_zpgy(&mut self, bus: &mut Bus) -> u8 {
        let address = bus.read(self.pc).wrapping_add(self.y) as u16;
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
        let address_ptr = read_u16(bus, self.pc);
        self.inc_pc();
        self.inc_pc();
        let lo = bus.read(address_ptr) as u16;
        // cpu wrapping bug
        let hi_ptr_hi = address_ptr & 0xff00;
        let hi_ptr_lo = address_ptr.wrapping_add(1) & 0x00ff;
        let hi = (bus.read(hi_ptr_lo + hi_ptr_hi) as u16) << 8;
        lo + hi
    }

    pub(crate) fn fetch_address_xind(&mut self, bus: &mut Bus) -> u16 {
        let operand = bus.read(self.pc);
        self.inc_pc();
        let address_ptr = operand.wrapping_add(self.x);
        let lo = bus.read(address_ptr as u16) as u16;
        let hi = (bus.read(address_ptr.wrapping_add(1) as u16) as u16) << 8;
        lo + hi
    }

    pub(crate) fn fetch_address_indy(&mut self, bus: &mut Bus) -> u16 {
        let address_ptr = bus.read(self.pc);
        self.inc_pc();
        let lo = bus.read(address_ptr as u16) as u16;
        let hi = (bus.read(address_ptr.wrapping_add(1) as u16) as u16) << 8;
        (lo + hi).wrapping_add(self.y as u16)
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
        println!("{:x}", address);
        self.inc_pc();
        address
    }
}

fn read_u16(bus: &mut Bus, addr: u16) -> u16 {
    let lo_byte = bus.read(addr) as u16;
    let hi_byte = bus.read(addr.wrapping_add(1)) as u16;
    (hi_byte << 8) | lo_byte
}
