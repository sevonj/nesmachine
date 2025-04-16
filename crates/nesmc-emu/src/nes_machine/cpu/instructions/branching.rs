use crate::{
    bus::Bus,
    nes_machine::cpu::{Cpu, status::CpuStatus},
};

impl Cpu {
    fn branch(&mut self, value: u8) {
        let signed = value as i8;
        self.pc = self.pc.wrapping_add_signed(signed as i16);
    }

    /// Branch if carry clear
    pub(super) fn instr_bcc_rel(&mut self, bus: &mut Bus) {
        let value = self.fetch_operand_rel(bus);
        if !self.status.c {
            self.branch(value);
        }
    }
    /// Branch if carry set
    pub(super) fn instr_bcs_rel(&mut self, bus: &mut Bus) {
        let value = self.fetch_operand_rel(bus);
        if self.status.c {
            self.branch(value);
        }
    }
    /// Branch if equal
    pub(super) fn instr_beq_rel(&mut self, bus: &mut Bus) {
        let value = self.fetch_operand_rel(bus);
        if self.status.z {
            self.branch(value);
        }
    }
    /// Branch not equal
    pub(super) fn instr_bne_rel(&mut self, bus: &mut Bus) {
        let value = self.fetch_operand_rel(bus);
        if !self.status.z {
            self.branch(value);
        }
    }
    /// Branch if plus (not negative)
    pub(super) fn instr_bpl_rel(&mut self, bus: &mut Bus) {
        let value = self.fetch_operand_rel(bus);
        if !self.status.n {
            self.branch(value);
        }
    }
    /// Branch if minus (negative)
    pub(super) fn instr_bmi_rel(&mut self, bus: &mut Bus) {
        let value = self.fetch_operand_rel(bus);
        if self.status.n {
            self.branch(value);
        }
    }
    /// Branch if overflow clear
    pub(super) fn instr_bvc_rel(&mut self, bus: &mut Bus) {
        let value = self.fetch_operand_rel(bus);
        if !self.status.v {
            self.branch(value);
        }
    }
    /// Branch if overflow set
    pub(super) fn instr_bvs_rel(&mut self, bus: &mut Bus) {
        let value = self.fetch_operand_rel(bus);
        if self.status.v {
            self.branch(value);
        }
    }

    /// Jump
    pub(super) fn instr_jmp_abs(&mut self, bus: &mut Bus) {
        self.pc = self.fetch_address_abs(bus);
    }
    /// Jump
    pub(super) fn instr_jmp_ind(&mut self, bus: &mut Bus) {
        self.pc = self.fetch_address_ind(bus);
    }

    /// Jump to subroutine
    pub(super) fn instr_jsr_abs(&mut self, bus: &mut Bus) {
        let value = self.fetch_address_abs(bus);
        let ret_addr = self.pc.wrapping_sub(1);
        let hi = (ret_addr >> 8) as u8;
        let lo = (ret_addr & 0xff) as u8;
        self.push_stack(hi, bus);
        self.push_stack(lo, bus);
        self.pc = value;
    }

    /// Return from subroutine
    pub(super) fn instr_rts_impl(&mut self, bus: &mut Bus) {
        let lo = self.pop_stack(bus) as u16;
        let hi = (self.pop_stack(bus) as u16) << 8;
        let ret_addr = (hi + lo).wrapping_add(1);
        self.pc = ret_addr;
    }

    /// Break - IRQ
    pub(super) fn instr_brk_impl(&mut self, bus: &mut Bus) {
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
    }

    /// IRQ Return
    pub(super) fn instr_rti_impl(&mut self, bus: &mut Bus) {
        self.status = CpuStatus::from(self.pop_stack(bus));

        let lo = self.pop_stack(bus) as u16;
        let hi = (self.pop_stack(bus) as u16) << 8;
        self.pc = lo + hi;
    }
}

fn read_u16(bus: &mut Bus, addr: u16) -> u16 {
    let lo_byte = bus.read(addr) as u16;
    let hi_byte = bus.read(addr.wrapping_add(1)) as u16;
    (hi_byte << 8) | lo_byte
}
