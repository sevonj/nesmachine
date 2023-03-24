use crate::emulator::Bus;

pub(crate) enum Mode {
    Accumulator,
    Absolute,
    AbsoluteX,
    AbsoluteY,
    Immediate,
    Implied,
    Indirect,
    XIndirect,
    IndirectY,
    Relative,
    ZeroPage,
    ZeroPageX,
    ZeroPageY,
}

use super::{Cpu, FLAG_B, FLAG_C, FLAG_D, FLAG_I, FLAG_N, FLAG_U, FLAG_V, FLAG_Z};

impl Cpu {
    fn set_negative(&mut self, result: u8) {
        match result & 0x80 != 0 {
            true => self.status |= FLAG_N,
            false => self.status &= !FLAG_N,
        }
    }
    fn set_overflow(&mut self, data: u8, result: u8) {
        let sign_a = self.a & 0x80 != 0;
        let sign_b = data & 0x80 != 0;
        let sign_res = result & 0x80 != 0;
        match (sign_a && sign_b && !sign_res) || (!sign_a && !sign_b && sign_res) {
            true => self.status |= FLAG_V,
            false => self.status &= !FLAG_V,
        }
    }
    fn set_carry(&mut self, yes: bool) {
        match yes {
            true => self.status |= FLAG_C,
            false => self.status &= !FLAG_C,
        }
    }
    fn set_int_disable(&mut self, yes: bool) {
        match yes {
            true => self.status |= FLAG_I,
            false => self.status &= !FLAG_I,
        }
    }
    fn set_zero(&mut self, result: u8) {
        match result == 0 {
            true => self.status |= FLAG_Z,
            false => self.status &= !FLAG_Z,
        }
    }
    fn branch(&mut self, target: i8) {
        let pc = self.pc as i16;
        self.pc = (pc + target as i16) as u16 & 0xffff;
    }
    pub(crate) fn push_stack(&mut self, data: u8, bus: &mut Bus) {
        bus.write(0x0100 + self.sp as u16, data);
        self.sp = self.sp.wrapping_sub(1);
    }
    pub(crate) fn pop_stack(&mut self, bus: &mut Bus) -> u8 {
        self.sp = self.sp.wrapping_add(1);
        bus.read(0x0100 + self.sp as u16)
    }
    // ----- Instructions
    // --- Misc
    pub(crate) fn instr_nop(&mut self) {}
    pub(crate) fn instr_nop_with_operand(&mut self, mode: Mode, bus: &mut Bus) {
        self.get_operand(mode, bus);
    }

    // --- Load/Store
    pub(crate) fn instr_lda(&mut self, mode: Mode, bus: &mut Bus) {
        let data = self.get_operand(mode, bus);
        self.a = data;
        self.set_zero(data);
        self.set_negative(data);
    }
    pub(crate) fn instr_ldx(&mut self, mode: Mode, bus: &mut Bus) {
        let data = self.get_operand(mode, bus);
        self.x = data;
        self.set_zero(data);
        self.set_negative(data);
    }
    pub(crate) fn instr_ldy(&mut self, mode: Mode, bus: &mut Bus) {
        let data = self.get_operand(mode, bus);
        self.y = data;
        self.set_zero(data);
        self.set_negative(data);
    }
    pub(crate) fn instr_sta(&mut self, mode: Mode, bus: &mut Bus) {
        let addr = self.get_address(mode, bus);
        bus.write(addr, self.a);
    }
    pub(crate) fn instr_stx(&mut self, mode: Mode, bus: &mut Bus) {
        let addr = self.get_address(mode, bus);
        bus.write(addr, self.x);
    }
    pub(crate) fn instr_sty(&mut self, mode: Mode, bus: &mut Bus) {
        let addr = self.get_address(mode, bus);
        bus.write(addr, self.y);
    }

    // --- Transfer
    pub(crate) fn instr_tax(&mut self, _mode: Mode, _bus: &mut Bus) {
        self.x = self.a;
        self.set_zero(self.x);
        self.set_negative(self.x);
    }
    pub(crate) fn instr_tay(&mut self, _mode: Mode, _bus: &mut Bus) {
        self.y = self.a;
        self.set_zero(self.y);
        self.set_negative(self.y);
    }
    pub(crate) fn instr_tsx(&mut self, _mode: Mode, _bus: &mut Bus) {
        self.x = self.sp;
        self.set_zero(self.x);
        self.set_negative(self.x);
    }
    pub(crate) fn instr_txa(&mut self, _mode: Mode, _bus: &mut Bus) {
        self.a = self.x;
        self.set_zero(self.a);
        self.set_negative(self.a);
    }
    pub(crate) fn instr_txs(&mut self, _mode: Mode, _bus: &mut Bus) {
        self.sp = self.x;
    }
    pub(crate) fn instr_tya(&mut self, _mode: Mode, _bus: &mut Bus) {
        self.a = self.y;
        self.set_zero(self.a);
        self.set_negative(self.a);
    }

    // --- Stack
    pub(crate) fn instr_pha(&mut self, mode: Mode, bus: &mut Bus) {
        self.push_stack(self.a, bus);
    }
    pub(crate) fn instr_php(&mut self, mode: Mode, bus: &mut Bus) {
        self.push_stack(self.status | FLAG_B | FLAG_U, bus);
    }
    pub(crate) fn instr_pla(&mut self, mode: Mode, bus: &mut Bus) {
        self.a = self.pop_stack(bus);
        self.set_zero(self.a);
        self.set_negative(self.a);
    }
    pub(crate) fn instr_plp(&mut self, mode: Mode, bus: &mut Bus) {
        let status = self.pop_stack(bus);
        self.status = status & !FLAG_B & !FLAG_U;
    }

    // --- Shift
    pub(crate) fn instr_asl(&mut self, mode: Mode, bus: &mut Bus) {
        match mode {
            Mode::Accumulator => {
                let shifted = self.a << 1;
                self.set_carry(self.a & 0x80 != 0);
                self.set_zero(shifted);
                self.set_negative(shifted);
                self.a = shifted;
            }
            _ => {
                let addr = self.get_address(mode, bus);
                let data = bus.read(addr);
                let shifted = data << 1;
                self.set_carry(data & 0x80 != 0);
                self.set_zero(shifted);
                self.set_negative(shifted);
                bus.write(addr, shifted);
            }
        }
    }
    pub(crate) fn instr_lsr(&mut self, mode: Mode, bus: &mut Bus) {
        match mode {
            Mode::Accumulator => {
                let shifted = self.a >> 1;
                self.set_carry(self.a & 0x01 != 0);
                self.set_zero(shifted);
                self.set_negative(shifted);
                self.a = shifted;
            }
            _ => {
                let addr = self.get_address(mode, bus);
                let data = bus.read(addr);
                let shifted = data >> 1;
                self.set_carry(data & 0x01 != 0);
                self.set_zero(shifted);
                self.set_negative(shifted);
                bus.write(addr, shifted);
            }
        }
    }
    pub(crate) fn instr_rol(&mut self, mode: Mode, bus: &mut Bus) {
        match mode {
            Mode::Accumulator => {
                let carry = match self.status & FLAG_C != 0 {
                    true => 0x01,
                    false => 0x00,
                };
                let shifted = (self.a << 1) | carry;
                self.set_carry(self.a & 0x80 != 0);
                self.set_zero(shifted);
                self.set_negative(shifted);
                self.a = shifted;
            }
            _ => {
                let addr = self.get_address(mode, bus);
                let data = bus.read(addr);
                let carry = match self.status & FLAG_C != 0 {
                    true => 0x01,
                    false => 0x00,
                };
                let shifted = (data << 1) | carry;
                self.set_carry(data & 0x80 != 0);
                self.set_zero(shifted);
                self.set_negative(shifted);
                bus.write(addr, shifted);
            }
        }
    }
    pub(crate) fn instr_ror(&mut self, mode: Mode, bus: &mut Bus) {
        match mode {
            Mode::Accumulator => {
                let carry = match self.status & FLAG_C != 0 {
                    true => 0x80,
                    false => 0x00,
                };
                let shifted = (self.a >> 1) | carry;
                self.set_carry(self.a & 0x01 != 0);
                self.set_zero(shifted);
                self.set_negative(shifted);
                self.a = shifted;
            }
            _ => {
                let addr = self.get_address(mode, bus);
                let data = bus.read(addr);
                let carry = match self.status & FLAG_C != 0 {
                    true => 0x80,
                    false => 0x00,
                };
                let shifted = (data >> 1) | carry;
                self.set_carry(data & 0x01 != 0);
                self.set_zero(shifted);
                self.set_negative(shifted);
                bus.write(addr, shifted);
            }
        }
    }

    // --- Logic
    pub(crate) fn instr_and(&mut self, mode: Mode, bus: &mut Bus) {
        let data = self.get_operand(mode, bus);
        let result = self.a & data;
        self.set_zero(result);
        self.set_negative(result);
        self.a = result;
    }

    pub(crate) fn instr_bit(&mut self, mode: Mode, bus: &mut Bus) {
        let data = self.get_operand(mode, bus);
        let result = self.a & data;
        self.set_zero(result);
        self.status = self.status & !FLAG_V | (data & FLAG_V);
        self.set_negative(data);
    }

    pub(crate) fn instr_eor(&mut self, mode: Mode, bus: &mut Bus) {
        let data = self.get_operand(mode, bus);
        let result = self.a ^ data;
        self.set_zero(result);
        self.set_negative(result);
        self.a = result;
    }

    pub(crate) fn instr_ora(&mut self, mode: Mode, bus: &mut Bus) {
        let data = self.get_operand(mode, bus);
        let result = self.a | data;
        self.set_zero(result);
        self.set_negative(result);
        self.a = result;
    }
    // --- Arithmetic
    pub(crate) fn instr_adc(&mut self, mode: Mode, bus: &mut Bus) {
        let data = self.get_operand(mode, bus);
        let carry = if self.status & FLAG_C != 0 { 1 } else { 0 };
        let (result, ov1) = self.a.overflowing_add(data); // add data
        let (result, ov2) = result.overflowing_add(carry); // add carry from previous operation
        self.set_carry(ov1 || ov2);
        self.set_zero(result);
        self.set_negative(result);
        self.set_overflow(data, result);
        self.a = result;
    }
    pub(crate) fn instr_sbc(&mut self, mode: Mode, bus: &mut Bus) {
        let data = self.get_operand(mode, bus);
        let carry = if self.status & FLAG_C != 0 { 1 } else { 0 };
        let (result, ov1) = self.a.overflowing_add(!data); // add data
        let (result, ov2) = result.overflowing_add(carry); // add carry from previous operation
        self.set_carry(result <= self.a);
        self.set_zero(result);
        self.set_negative(result);
        self.set_overflow(!data, result);
        self.a = result;
    }
    pub(crate) fn instr_cmp(&mut self, mode: Mode, bus: &mut Bus) {
        let data = self.get_operand(mode, bus);
        let result = self.a.wrapping_sub(data);
        self.set_carry(self.a >= data);
        self.set_zero(result);
        self.set_negative(result);
    }
    pub(crate) fn instr_cpx(&mut self, mode: Mode, bus: &mut Bus) {
        let data = self.get_operand(mode, bus);
        let result = self.x.wrapping_sub(data);
        self.set_carry(self.x >= data);
        self.set_zero(result);
        self.set_negative(result);
    }
    pub(crate) fn instr_cpy(&mut self, mode: Mode, bus: &mut Bus) {
        let data = self.get_operand(mode, bus);
        let result = self.y.wrapping_sub(data);
        self.set_carry(self.y >= data);
        self.set_zero(result);
        self.set_negative(result);
    }

    // --- IncDec
    pub(crate) fn instr_dec(&mut self, mode: Mode, bus: &mut Bus) {
        let addr = self.get_address(mode, bus);
        let value = bus.read(addr).wrapping_sub(1);
        self.set_zero(value);
        self.set_negative(value);
        bus.write(addr, value);
    }
    pub(crate) fn instr_dex(&mut self, mode: Mode, bus: &mut Bus) {
        self.x = self.x.wrapping_sub(1);
        self.set_zero(self.x);
        self.set_negative(self.x);
    }
    pub(crate) fn instr_dey(&mut self, mode: Mode, bus: &mut Bus) {
        self.y = self.y.wrapping_sub(1);
        self.set_zero(self.y);
        self.set_negative(self.y);
    }
    pub(crate) fn instr_inc(&mut self, mode: Mode, bus: &mut Bus) {
        let addr = self.get_address(mode, bus);
        let value = bus.read(addr).wrapping_add(1);
        self.set_zero(value);
        self.set_negative(value);
        bus.write(addr, value);
    }
    pub(crate) fn instr_inx(&mut self, mode: Mode, bus: &mut Bus) {
        self.x = self.x.wrapping_add(1);
        self.set_zero(self.x);
        self.set_negative(self.x);
    }
    pub(crate) fn instr_iny(&mut self, mode: Mode, bus: &mut Bus) {
        self.y = self.y.wrapping_add(1);
        self.set_zero(self.y);
        self.set_negative(self.y);
    }

    // --- Control
    pub(crate) fn instr_jmp(&mut self, mode: Mode, bus: &mut Bus) {
        let addr = self.get_address(mode, bus);
        self.pc = addr;
    }
    pub(crate) fn instr_jsr(&mut self, mode: Mode, bus: &mut Bus) {
        let addr = self.get_address(mode, bus);
        let ret_addr = self.pc - 1;
        let hi = (ret_addr >> 8) as u8;
        let lo = (ret_addr & 0xff) as u8;
        self.push_stack(hi, bus);
        self.push_stack(lo, bus);
        self.pc = addr;
    }
    pub(crate) fn instr_rts(&mut self, mode: Mode, bus: &mut Bus) {
        let lo = self.pop_stack(bus);
        let hi = self.pop_stack(bus);
        let ret_addr = ((hi as u16) << 8) + lo as u16;
        self.pc = ret_addr + 1;
    }
    pub(crate) fn instr_brk(&mut self, bus: &mut Bus) {
        let ret_addr = self.pc;
        let hi = (ret_addr >> 8) as u8;
        let lo = (ret_addr & 0xff) as u8;

        self.push_stack(hi, bus);
        self.push_stack(lo, bus);
        self.status |= FLAG_B | FLAG_U;

        bus.write(0x0100 + self.sp as u16, self.status);
        self.set_int_disable(true);
        let irq_vec = 0; // bus.read_word(IRQ_VECTOR);
        self.pc = irq_vec;
    }
    pub(crate) fn instr_rti(&mut self, bus: &mut Bus) {
        self.status = self.pop_stack(bus);
        let lo = self.pop_stack(bus);
        let hi = self.pop_stack(bus);
        let ret_addr = ((hi as u16) << 8) + lo as u16;
        self.pc = ret_addr;
    }

    // --- Branching
    pub(crate) fn instr_bcc(&mut self, mode: Mode, bus: &mut Bus) {
        let target = self.get_operand(mode, bus) as i8;
        if self.status & FLAG_C == 0 {
            self.branch(target);
        }
    }
    pub(crate) fn instr_bcs(&mut self, mode: Mode, bus: &mut Bus) {
        let target = self.get_operand(mode, bus) as i8;
        if self.status & FLAG_C != 0 {
            self.branch(target);
        }
    }
    pub(crate) fn instr_beq(&mut self, mode: Mode, bus: &mut Bus) {
        let target = self.get_operand(mode, bus) as i8;
        if self.status & FLAG_Z != 0 {
            self.branch(target);
        }
    }
    pub(crate) fn instr_bmi(&mut self, mode: Mode, bus: &mut Bus) {
        let target = self.get_operand(mode, bus) as i8;
        if self.status & FLAG_N != 0 {
            self.branch(target);
        }
    }
    pub(crate) fn instr_bne(&mut self, mode: Mode, bus: &mut Bus) {
        let target = self.get_operand(mode, bus) as i8;
        if self.status & FLAG_Z == 0 {
            self.branch(target);
        }
    }
    pub(crate) fn instr_bpl(&mut self, mode: Mode, bus: &mut Bus) {
        let target = self.get_operand(mode, bus) as i8;
        if self.status & FLAG_N == 0 {
            self.branch(target);
        }
    }
    pub(crate) fn instr_bvc(&mut self, mode: Mode, bus: &mut Bus) {
        let target = self.get_operand(mode, bus) as i8;
        if self.status & FLAG_V == 0 {
            self.branch(target);
        }
    }
    pub(crate) fn instr_bvs(&mut self, mode: Mode, bus: &mut Bus) {
        let target = self.get_operand(mode, bus) as i8;
        if self.status & FLAG_V != 0 {
            self.branch(target);
        }
    }
    // --- Flags
    pub(crate) fn instr_clc(&mut self) {
        self.status &= !FLAG_C;
    }
    pub(crate) fn instr_cld(&mut self) {
        self.status &= !FLAG_D;
    }
    pub(crate) fn instr_cli(&mut self) {
        self.status &= !FLAG_I;
    }
    pub(crate) fn instr_clv(&mut self) {
        self.status &= !FLAG_V;
    }
    pub(crate) fn instr_sec(&mut self) {
        self.status |= FLAG_C;
    }
    pub(crate) fn instr_sed(&mut self) {
        self.status |= FLAG_D;
    }
    pub(crate) fn instr_sei(&mut self) {
        self.status |= FLAG_I;
    }

    // --- Illegal
    pub(crate) fn instr_alr(&mut self, mode: Mode, bus: &mut Bus) {
        let data = self.get_operand(mode, bus);
        let and_result = self.a & data;
        self.a = and_result >> 1;
        self.set_carry(and_result & 0x01 != 0);
        self.set_zero(self.a);
        self.set_negative(self.a);
    }
    pub(crate) fn instr_anc(&mut self, mode: Mode, bus: &mut Bus) {
        let data = self.get_operand(mode, bus);
        self.a &= data;
        self.set_negative(self.a);
        self.set_zero(self.a);
        self.set_carry(self.status & FLAG_N != 0);
    }
    pub(crate) fn instr_ane(&mut self, mode: Mode, bus: &mut Bus) {}
    pub(crate) fn instr_arr(&mut self, mode: Mode, bus: &mut Bus) {
        let data = self.get_operand(mode, bus);
        let and_result = self.a & data;
        self.a = and_result >> 1;
        self.set_carry(and_result & 0x01 != 0);
        let xor_result = self.a ^ data;
        self.set_overflow(data, xor_result);
        self.set_negative(self.a);
        self.set_zero(self.a);
        self.set_carry(xor_result & 0x80 != 0);
    }
    pub(crate) fn instr_dcp(&mut self, mode: Mode, bus: &mut Bus) {
        let addr = self.get_address(mode, bus);
        let data = bus.read(addr);
        let result = data.wrapping_sub(1);
        self.set_zero(result);
        self.set_negative(result);
        bus.write(addr, result);
        let result = self.a.wrapping_sub(result);
        self.set_carry(self.a >= result);
        self.set_zero(result);
        self.set_negative(result);
    }
    pub(crate) fn instr_isc(&mut self, mode: Mode, bus: &mut Bus) {
        let addr = self.get_address(mode, bus);
        let data = bus.read(addr).wrapping_add(1);
        bus.write(addr, data);
        let carry = if self.status & FLAG_C != 0 { 1 } else { 0 };
        let (result, ov1) = self.a.overflowing_add(!data); // add data
        let (result, ov2) = result.overflowing_add(carry); // add carry from previous operation
        self.set_carry(result <= self.a);
        self.set_zero(result);
        self.set_negative(result);
        self.set_overflow(!data, result);
        self.a = result;
    }
    pub(crate) fn instr_las(&mut self, mode: Mode, bus: &mut Bus) {
        panic!("Illegal instruction: LAS");
    }

    pub(crate) fn instr_lax(&mut self, mode: Mode, bus: &mut Bus) {
        let data = self.get_operand(mode, bus);
        self.a = data;
        self.x = data;
        self.set_zero(data);
        self.set_negative(data);
    }

    pub(crate) fn instr_lxa(&mut self, mode: Mode, bus: &mut Bus) {
        let data = self.get_operand(mode, bus);
        self.a = self.a & self.x & data;
        self.x = self.a;
        self.set_zero(self.a);
        self.set_negative(self.a);
    }
    pub(crate) fn instr_rla(&mut self, mode: Mode, bus: &mut Bus) {
        let addr = self.get_address(mode, bus);
        let data = bus.read(addr);
        let carry = match self.status & FLAG_C != 0 {
            true => 0x01,
            false => 0x00,
        };
        let shifted = (data << 1) | carry;
        self.set_carry(data & 0x80 != 0);
        self.set_zero(shifted);
        self.set_negative(shifted);
        bus.write(addr, shifted);

        let result = self.a & shifted;
        self.set_zero(result);
        self.set_negative(result);
        self.a = result;
    }
    pub(crate) fn instr_rra(&mut self, mode: Mode, bus: &mut Bus) {
        let addr = self.get_address(mode, bus);
        let data = bus.read(addr);
        let carry = match self.status & FLAG_C != 0 {
            true => 0x80,
            false => 0x00,
        };
        let shifted = (data >> 1) | carry;
        self.set_carry(data & 0x01 != 0);
        self.set_zero(shifted);
        self.set_negative(shifted);
        bus.write(addr, shifted);

        let data = shifted;
        let carry = if self.status & FLAG_C != 0 { 1 } else { 0 };
        let (result, ov1) = self.a.overflowing_add(data); // add data
        let (result, ov2) = result.overflowing_add(carry); // add carry from previous operation
        self.set_carry(ov1 || ov2);
        self.set_zero(result);
        self.set_negative(result);
        self.set_overflow(data, result);
        self.a = result;
    }
    pub(crate) fn instr_sax(&mut self, mode: Mode, bus: &mut Bus) {
        let addr = self.get_address(mode, bus);
        let data = self.a & self.x;
        bus.write(addr, data);
    }
    pub(crate) fn instr_sbx(&mut self, mode: Mode, bus: &mut Bus) {
        let data = self.get_operand(mode, bus);
        let result = self.a & self.x;
        let borrow = if result >= data { 0 } else { 1 };
        self.x = result.wrapping_sub(data).wrapping_sub(borrow);
        self.set_zero(self.x);
        self.set_negative(self.x);
    }
    pub(crate) fn instr_sha(&mut self, mode: Mode, bus: &mut Bus) {
        let addr = self.get_address(mode, bus);
        let data = self.a & self.x;
        let high = (addr >> 8) as u8 & 0xFF;
        let result = data.wrapping_add(high);
        bus.write(addr, result);
    }
    pub(crate) fn instr_shx(&mut self, mode: Mode, bus: &mut Bus) {
        let addr = self.get_address(mode, bus);
        let data = self.x;
        let high = (addr >> 8) as u8 & 0xFF;
        let result = data & high;
        bus.write(addr, result);
    }
    pub(crate) fn instr_shy(&mut self, mode: Mode, bus: &mut Bus) {
        let addr = self.get_address(mode, bus);
        let data = self.y;
        let high = (addr >> 8) as u8 & 0xFF;
        let result = data & high;
        bus.write(addr, result);
    }
    pub(crate) fn instr_slo(&mut self, mode: Mode, bus: &mut Bus) {
        let addr = self.get_address(mode, bus);
        let data = bus.read(addr);
        let result = data << 1;
        bus.write(addr, result);
        self.a |= result;
        self.set_zero(self.a);
        self.set_negative(self.a);
        self.set_carry((data & 0x80) != 0);
    }
    pub(crate) fn instr_sre(&mut self, mode: Mode, bus: &mut Bus) {
        let addr = self.get_address(mode, bus);
        let data = bus.read(addr);
        let result = data >> 1;
        bus.write(addr, result);
        self.a ^= result;
        self.set_zero(self.a);
        self.set_negative(self.a);
        self.set_carry((data & 0x01) != 0);
    }
    pub(crate) fn instr_tas(&mut self, mode: Mode, bus: &mut Bus) {}
    pub(crate) fn instr_usbc(&mut self, mode: Mode, bus: &mut Bus) {
        self.instr_sbc(mode, bus)
    }
    pub(crate) fn instr_jam(&mut self) {
        panic!("jam")
    }
}
