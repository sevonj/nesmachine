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

    pub(super) fn instr_lda_abs(&mut self, bus: &mut Bus) -> usize {
        let value = self.fetch_operand_abs(bus);
        self.instr_lda(value);
        4
    }
    pub(super) fn instr_lda_absx(&mut self, bus: &mut Bus) -> usize {
        let mut cycles = 4;
        let addr = self.fetch_address_absx(bus);
        // Page boundary crossed
        if addr & 0xff < self.x as u16 {
            cycles += 1
        }
        self.instr_lda(bus.read(addr));
        cycles
    }
    pub(super) fn instr_lda_absy(&mut self, bus: &mut Bus) -> usize {
        let mut cycles = 4;
        let addr = self.fetch_address_absy(bus);
        // Page boundary crossed
        if addr & 0xff < self.y as u16 {
            cycles += 1
        }
        self.instr_lda(bus.read(addr));
        cycles
    }
    pub(super) fn instr_lda_imm(&mut self, bus: &mut Bus) -> usize {
        let value = self.fetch_operand_imm(bus);
        self.instr_lda(value);
        2
    }
    pub(super) fn instr_lda_xind(&mut self, bus: &mut Bus) -> usize {
        let value = self.fetch_operand_xind(bus);
        self.instr_lda(value);
        6
    }
    pub(super) fn instr_lda_indy(&mut self, bus: &mut Bus) -> usize {
        let mut cycles = 5;
        let addr = self.fetch_address_indy(bus);
        // Page boundary crossed
        if addr & 0xff < self.y as u16 {
            cycles += 1
        }
        self.instr_lda(bus.read(addr));
        cycles
    }
    pub(super) fn instr_lda_zpg(&mut self, bus: &mut Bus) -> usize {
        let value = self.fetch_operand_zpg(bus);
        self.instr_lda(value);
        3
    }
    pub(super) fn instr_lda_zpgx(&mut self, bus: &mut Bus) -> usize {
        let value = self.fetch_operand_zpgx(bus);
        self.instr_lda(value);
        4
    }

    pub(super) fn instr_ldx_abs(&mut self, bus: &mut Bus) -> usize {
        let value = self.fetch_operand_abs(bus);
        self.instr_ldx(value);
        4
    }
    pub(super) fn instr_ldx_absy(&mut self, bus: &mut Bus) -> usize {
        let mut cycles = 4;
        let addr = self.fetch_address_absy(bus);
        // Page boundary crossed
        if addr & 0xff < self.y as u16 {
            cycles += 1
        }
        self.instr_ldx(bus.read(addr));
        cycles
    }
    pub(super) fn instr_ldx_imm(&mut self, bus: &mut Bus) -> usize {
        let value = self.fetch_operand_imm(bus);
        self.instr_ldx(value);
        2
    }
    pub(super) fn instr_ldx_zpg(&mut self, bus: &mut Bus) -> usize {
        let value = self.fetch_operand_zpg(bus);
        self.instr_ldx(value);
        3
    }
    pub(super) fn instr_ldx_zpgy(&mut self, bus: &mut Bus) -> usize {
        let value = self.fetch_operand_zpgy(bus);
        self.instr_ldx(value);
        4
    }

    pub(super) fn instr_ldy_abs(&mut self, bus: &mut Bus) -> usize {
        let value = self.fetch_operand_abs(bus);
        self.instr_ldy(value);
        4
    }
    pub(super) fn instr_ldy_absx(&mut self, bus: &mut Bus) -> usize {
        let mut cycles = 4;
        let addr = self.fetch_address_absx(bus);
        // Page boundary crossed
        if addr & 0xff < self.x as u16 {
            cycles += 1
        }
        self.instr_ldy(bus.read(addr));
        cycles
    }
    pub(super) fn instr_ldy_imm(&mut self, bus: &mut Bus) -> usize {
        let value = self.fetch_operand_imm(bus);
        self.instr_ldy(value);
        2
    }
    pub(super) fn instr_ldy_zpg(&mut self, bus: &mut Bus) -> usize {
        let value = self.fetch_operand_zpg(bus);
        self.instr_ldy(value);
        3
    }
    pub(super) fn instr_ldy_zpgx(&mut self, bus: &mut Bus) -> usize {
        let value = self.fetch_operand_zpgx(bus);
        self.instr_ldy(value);
        4
    }

    pub(super) fn instr_sta_abs(&mut self, bus: &mut Bus) -> usize {
        let addr = self.fetch_address_abs(bus);
        bus.write(addr, self.a);
        4
    }
    pub(super) fn instr_sta_absx(&mut self, bus: &mut Bus) -> usize {
        let addr = self.fetch_address_absx(bus);
        bus.write(addr, self.a);
        5
    }
    pub(super) fn instr_sta_absy(&mut self, bus: &mut Bus) -> usize {
        let addr = self.fetch_address_absy(bus);
        bus.write(addr, self.a);
        5
    }
    pub(super) fn instr_sta_xind(&mut self, bus: &mut Bus) -> usize {
        let addr = self.fetch_address_xind(bus);
        bus.write(addr, self.a);
        6
    }
    pub(super) fn instr_sta_indy(&mut self, bus: &mut Bus) -> usize {
        let addr = self.fetch_address_indy(bus);
        bus.write(addr, self.a);
        6
    }
    pub(super) fn instr_sta_zpg(&mut self, bus: &mut Bus) -> usize {
        let addr = self.fetch_address_zpg(bus);
        bus.write(addr, self.a);
        3
    }
    pub(super) fn instr_sta_zpgx(&mut self, bus: &mut Bus) -> usize {
        let addr = self.fetch_address_zpgx(bus);
        bus.write(addr, self.a);
        4
    }

    pub(super) fn instr_stx_abs(&mut self, bus: &mut Bus) -> usize {
        let addr = self.fetch_address_abs(bus);
        bus.write(addr, self.x);
        4
    }
    pub(super) fn instr_stx_zpg(&mut self, bus: &mut Bus) -> usize {
        let addr = self.fetch_address_zpg(bus);
        bus.write(addr, self.x);
        3
    }
    pub(super) fn instr_stx_zpgy(&mut self, bus: &mut Bus) -> usize {
        let addr = self.fetch_address_zpgy(bus);
        bus.write(addr, self.x);
        4
    }

    pub(super) fn instr_sty_abs(&mut self, bus: &mut Bus) -> usize {
        let addr = self.fetch_address_abs(bus);
        bus.write(addr, self.y);
        4
    }
    pub(super) fn instr_sty_zpg(&mut self, bus: &mut Bus) -> usize {
        let addr = self.fetch_address_zpg(bus);
        bus.write(addr, self.y);
        3
    }
    pub(super) fn instr_sty_zpgx(&mut self, bus: &mut Bus) -> usize {
        let addr = self.fetch_address_zpgx(bus);
        bus.write(addr, self.y);
        4
    }

    pub(super) fn instr_tax_impl(&mut self) -> usize {
        self.x = self.a;
        self.set_zero(self.x);
        self.set_negative(self.x);
        2
    }

    pub(super) fn instr_tay_impl(&mut self) -> usize {
        self.y = self.a;
        self.set_zero(self.y);
        self.set_negative(self.y);
        2
    }

    pub(super) fn instr_tsx_impl(&mut self) -> usize {
        self.x = self.sp;
        self.set_zero(self.x);
        self.set_negative(self.x);
        2
    }

    pub(super) fn instr_txa_impl(&mut self) -> usize {
        self.a = self.x;
        self.set_zero(self.a);
        self.set_negative(self.a);
        2
    }

    pub(super) fn instr_txs_impl(&mut self) -> usize {
        self.sp = self.x;
        2
    }

    pub(super) fn instr_tya_impl(&mut self) -> usize {
        self.a = self.y;
        self.set_zero(self.a);
        self.set_negative(self.a);
        2
    }

    /// Push A to stack
    pub(super) fn instr_pha_impl(&mut self, bus: &mut Bus) -> usize {
        self.push_stack(self.a, bus);
        3
    }

    /// Pull A from stack
    pub(super) fn instr_pla_impl(&mut self, bus: &mut Bus) -> usize {
        self.a = self.pop_stack(bus);
        self.set_zero(self.a);
        self.set_negative(self.a);
        4
    }

    /// Push status to stack
    pub(super) fn instr_php_impl(&mut self, bus: &mut Bus) -> usize {
        const BRK_FLAG: u8 = 0x10;
        let value = u8::from(self.status) | BRK_FLAG;
        self.push_stack(value, bus);
        3
    }

    /// Pull status from stack
    pub(super) fn instr_plp_impl(&mut self, bus: &mut Bus) -> usize {
        self.status = CpuStatus::from(self.pop_stack(bus));
        // TODO: I flag should be delayed by 1 inst
        4
    }
}
