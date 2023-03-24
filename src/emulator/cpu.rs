use self::instructions::Mode;
use super::Bus;
mod instruction_match;
mod instructions;

pub(crate) const FLAG_C: u8 = 0b0000_0001;
pub(crate) const FLAG_Z: u8 = 0b0000_0010;
pub(crate) const FLAG_I: u8 = 0b0000_0100;
pub(crate) const FLAG_D: u8 = 0b0000_1000;
pub(crate) const FLAG_B: u8 = 0b0001_0000;
pub(crate) const FLAG_U: u8 = 0b0010_0000;
pub(crate) const FLAG_V: u8 = 0b0100_0000;
pub(crate) const FLAG_N: u8 = 0b1000_0000;

pub(crate) struct Cpu {
    pub(crate) pc: u16,    // program counter
    pub(crate) a: u8,      // accumulator
    pub(crate) x: u8,      // x register
    pub(crate) y: u8,      // y register
    pub(crate) sp: u8,     // stack pointer
    pub(crate) status: u8, // status register
}

impl Cpu {
    pub(crate) fn new() -> Cpu {
        Cpu {
            pc: 0,
            a: 0,
            x: 0,
            y: 0,
            sp: 0,
            status: 0,
        }
    }

    pub(crate) fn reset(&mut self, bus: &mut Bus) {
        self.pc = (bus.read(0xFFFD) as u16) << 8 | bus.read(0xFFFC) as u16;
        self.a = 0;
        self.x = 0;
        self.y = 0;
        self.sp = 0xFD;
        self.status = 0b100100;

        //self.pc = 0xc000; // for nestest automation
    }

    pub(crate) fn tick(&mut self, bus: &mut Bus) {
        // vblank interrupt
        /*let ppu_status = bus.read(0x2002);
        if ppu_status & 0b1000_0000 != 0 {
            bus.write(0x2002, ppu_status & 0b0111_1111);
            self.nmi(bus);
            return;
        }*/
        // Fetch instruction
        let opcode = bus.read(self.pc);
        self.pc += 1;
        self.exec_opcode(bus, opcode)
    }

    pub(crate) fn nmi(&mut self, bus: &mut Bus) {
        // 1. Push PC and P onto stack
        let pc = self.pc;
        let p = self.sp & 0b1101_1111; // Clear B flag
        self.push_stack((pc >> 8) as u8, bus);
        self.push_stack(pc as u8, bus);
        self.push_stack(p, bus);

        // 2. Set PC to NMI handler address
        let nmi_addr_low = bus.read(0xFFFA) as u16;
        let nmi_addr_high = bus.read(0xFFFB) as u16;
        self.pc = (nmi_addr_high << 8) | nmi_addr_low;

        // 3. Execute NMI handler code
    }

    pub(crate) fn get_operand(&mut self, mode: Mode, bus: &mut Bus) -> u8 {
        match mode {
            Mode::Immediate => {
                let operand = bus.read(self.pc);
                self.pc += 1;
                operand
            }
            Mode::ZeroPage => {
                let address = bus.read(self.pc) as u16;
                self.pc += 1;
                bus.read(address)
            }
            Mode::ZeroPageX => {
                let address = bus.read(self.pc).wrapping_add(self.x) as u16;
                self.pc += 1;
                bus.read(address)
            }
            Mode::ZeroPageY => {
                let address = bus.read(self.pc).wrapping_add(self.y) as u16;
                self.pc += 1;
                bus.read(address)
            }
            Mode::Absolute => {
                let address = bus.read_u16(self.pc);
                self.pc += 2;
                bus.read(address)
            }
            Mode::AbsoluteX => {
                let address = bus.read_u16(self.pc).wrapping_add(self.x as u16);
                self.pc += 2;
                bus.read(address)
            }
            Mode::AbsoluteY => {
                let address = bus.read_u16(self.pc).wrapping_add(self.y as u16);
                self.pc += 2;
                bus.read(address)
            }
            Mode::Indirect => {
                panic!("Fetch operand: indirect. No instruction should call this.");
            }
            Mode::XIndirect => {
                let address = bus.read(self.pc).wrapping_add(self.x) as u16;
                self.pc += 1;
                let indirect_address = bus.read_u16_wrapped(address);
                bus.read(indirect_address)
            }
            Mode::IndirectY => {
                let address = bus.read(self.pc);
                self.pc += 1;
                let indirect_address = bus.read_u16_wrapped(address as u16);
                let address = indirect_address.wrapping_add(self.y as u16);
                bus.read(address)
            }
            Mode::Relative => {
                let operand = bus.read(self.pc);
                self.pc += 1;
                operand
            }
            Mode::Accumulator => self.a,
            Mode::Implied => {
                panic!("Tried to Fetch Implied")
            }
        }
    }
    pub(crate) fn get_address(&mut self, mode: Mode, bus: &mut Bus) -> u16 {
        match mode {
            Mode::ZeroPage => {
                let address = bus.read(self.pc) as u16;
                self.pc += 1;
                address
            }
            Mode::ZeroPageX => {
                let address = bus.read(self.pc).wrapping_add(self.x) as u16;
                self.pc += 1;
                address
            }
            Mode::ZeroPageY => {
                let address = bus.read(self.pc).wrapping_add(self.y) as u16;
                self.pc += 1;
                address
            }
            Mode::Absolute => {
                let address = bus.read_u16(self.pc);
                self.pc += 2;
                address
            }
            Mode::AbsoluteX => {
                let address = bus.read_u16(self.pc).wrapping_add(self.x as u16);
                self.pc += 2;
                address
            }
            Mode::AbsoluteY => {
                let address = bus.read_u16(self.pc).wrapping_add(self.y as u16);
                self.pc += 2;
                address
            }
            Mode::Indirect => {
                let address = bus.read_u16_wrapped(self.pc);
                self.pc += 2;
                bus.read_u16_wrapped(address)
            }
            Mode::XIndirect => {
                let address = bus.read(self.pc).wrapping_add(self.x);
                self.pc += 1;
                bus.read_u16_wrapped(address as u16)
            }
            Mode::IndirectY => {
                let address = bus.read(self.pc);
                self.pc += 1;
                let indirect_address = bus.read_u16_wrapped(address as u16); //read_u16_wrapped
                indirect_address.wrapping_add(self.y as u16)
            }
            _ => panic!("Fetch address: Invalid mode. pc = {:#06x}", self.pc),
        }
    }
}
