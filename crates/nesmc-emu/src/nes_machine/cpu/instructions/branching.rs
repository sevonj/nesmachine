use crate::{
    bus::Bus,
    nes_machine::cpu::{Cpu, status::CpuStatus},
};

impl Cpu {
    fn branch(&mut self, value: u8) -> usize {
        let signed = value as i8;
        let old_page = self.pc & 0xff00;
        self.pc = self.pc.wrapping_add_signed(signed as i16);
        // +1 cycle if branch to same page.
        // +2 if different page.
        if self.pc & 0xff00 == old_page { 1 } else { 2 }
    }

    /// Branch if carry clear
    pub(super) fn instr_bcc_rel(&mut self, bus: &mut Bus) -> usize {
        let value = self.fetch_operand_rel(bus);
        let mut cycles = 2;
        if !self.status.c {
            cycles += self.branch(value);
        }
        cycles
    }
    /// Branch if carry set
    pub(super) fn instr_bcs_rel(&mut self, bus: &mut Bus) -> usize {
        let value = self.fetch_operand_rel(bus);
        let mut cycles = 2;
        if self.status.c {
            cycles += self.branch(value);
        }
        cycles
    }
    /// Branch if equal
    pub(super) fn instr_beq_rel(&mut self, bus: &mut Bus) -> usize {
        let value = self.fetch_operand_rel(bus);
        let mut cycles = 2;
        if self.status.z {
            cycles += self.branch(value);
        }
        cycles
    }
    /// Branch not equal
    pub(super) fn instr_bne_rel(&mut self, bus: &mut Bus) -> usize {
        let value = self.fetch_operand_rel(bus);
        let mut cycles = 2;
        if !self.status.z {
            cycles += self.branch(value);
        }
        cycles
    }
    /// Branch if plus (not negative)
    pub(super) fn instr_bpl_rel(&mut self, bus: &mut Bus) -> usize {
        let value = self.fetch_operand_rel(bus);
        let mut cycles = 2;
        if !self.status.n {
            cycles += self.branch(value);
        }
        cycles
    }
    /// Branch if minus (negative)
    pub(super) fn instr_bmi_rel(&mut self, bus: &mut Bus) -> usize {
        let value = self.fetch_operand_rel(bus);
        let mut cycles = 2;
        if self.status.n {
            cycles += self.branch(value);
        }
        cycles
    }
    /// Branch if overflow clear
    pub(super) fn instr_bvc_rel(&mut self, bus: &mut Bus) -> usize {
        let value = self.fetch_operand_rel(bus);
        let mut cycles = 2;
        if !self.status.v {
            cycles += self.branch(value);
        }
        cycles
    }
    /// Branch if overflow set
    pub(super) fn instr_bvs_rel(&mut self, bus: &mut Bus) -> usize {
        let value = self.fetch_operand_rel(bus);
        let mut cycles = 2;
        if self.status.v {
            cycles += self.branch(value);
        }
        cycles
    }

    /// Jump
    pub(super) fn instr_jmp_abs(&mut self, bus: &mut Bus) -> usize {
        self.pc = self.fetch_address_abs(bus);
        3
    }
    /// Jump
    pub(super) fn instr_jmp_ind(&mut self, bus: &mut Bus) -> usize {
        self.pc = self.fetch_address_ind(bus);
        5
    }

    /// Jump to subroutine
    pub(super) fn instr_jsr_abs(&mut self, bus: &mut Bus) -> usize {
        let value = self.fetch_address_abs(bus);
        let ret_addr = self.pc.wrapping_sub(1);
        let hi = (ret_addr >> 8) as u8;
        let lo = (ret_addr & 0xff) as u8;
        self.push_stack(hi, bus);
        self.push_stack(lo, bus);
        self.pc = value;
        6
    }

    /// Return from subroutine
    pub(super) fn instr_rts_impl(&mut self, bus: &mut Bus) -> usize {
        let lo = self.pop_stack(bus) as u16;
        let hi = (self.pop_stack(bus) as u16) << 8;
        let ret_addr = (hi + lo).wrapping_add(1);
        self.pc = ret_addr;
        6
    }

    /// Break - IRQ
    pub(super) fn instr_brk_impl(&mut self, bus: &mut Bus) -> usize {
        const BRK_FLAG: u8 = 0x10;

        let ret_addr = self.pc;
        let hi = (ret_addr << 8) as u8;
        let lo = (ret_addr & 0xff) as u8;
        self.push_stack(hi, bus);
        self.push_stack(lo, bus);

        let flags = u8::from(self.status) | BRK_FLAG;
        self.push_stack(flags, bus);

        self.status.i = true;
        self.pc = read_u16(bus, 0xfffe);
        7
    }

    /// IRQ Return
    pub(super) fn instr_rti_impl(&mut self, bus: &mut Bus) -> usize {
        self.status = CpuStatus::from(self.pop_stack(bus));

        let lo = self.pop_stack(bus) as u16;
        let hi = (self.pop_stack(bus) as u16) << 8;
        self.pc = lo + hi;
        6
    }
}

fn read_u16(bus: &mut Bus, addr: u16) -> u16 {
    let lo_byte = bus.read(addr) as u16;
    let hi_byte = bus.read(addr.wrapping_add(1)) as u16;
    (hi_byte << 8) | lo_byte
}
