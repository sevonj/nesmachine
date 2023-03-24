use super::{instructions::Mode, Cpu};
use crate::emulator::Bus;

impl Cpu {
    pub(crate) fn exec_opcode(&mut self, bus: &mut Bus, opcode: u8) {
        match opcode {
            0x00 => self.instr_brk(bus),
            0x01 => self.instr_ora(Mode::XIndirect, bus),
            0x02 => self.instr_jam(),
            0x03 => self.instr_slo(Mode::XIndirect, bus),
            0x04 => self.instr_nop_with_operand(Mode::ZeroPage, bus),
            0x05 => self.instr_ora(Mode::ZeroPage, bus),
            0x06 => self.instr_asl(Mode::ZeroPage, bus),
            0x07 => self.instr_slo(Mode::ZeroPage, bus),
            0x08 => self.instr_php(Mode::Implied, bus),
            0x09 => self.instr_ora(Mode::Immediate, bus),
            0x0a => self.instr_asl(Mode::Accumulator, bus),
            0x0b => self.instr_anc(Mode::Immediate, bus),
            0x0c => self.instr_nop_with_operand(Mode::Absolute, bus),
            0x0d => self.instr_ora(Mode::Absolute, bus),
            0x0e => self.instr_asl(Mode::Absolute, bus),
            0x0f => self.instr_slo(Mode::Absolute, bus),
            0x10 => self.instr_bpl(Mode::Relative, bus),
            0x11 => self.instr_ora(Mode::IndirectY, bus),
            0x12 => self.instr_jam(),
            0x13 => self.instr_slo(Mode::IndirectY, bus),
            0x14 => self.instr_nop_with_operand(Mode::ZeroPageX, bus),
            0x15 => self.instr_ora(Mode::ZeroPageX, bus),
            0x16 => self.instr_asl(Mode::ZeroPageX, bus),
            0x17 => self.instr_slo(Mode::ZeroPageX, bus),
            0x18 => self.instr_clc(),
            0x19 => self.instr_ora(Mode::AbsoluteY, bus),
            0x1a => self.instr_nop(),
            0x1b => self.instr_slo(Mode::AbsoluteY, bus),
            0x1c => self.instr_nop_with_operand(Mode::AbsoluteX, bus),
            0x1d => self.instr_ora(Mode::AbsoluteX, bus),
            0x1e => self.instr_asl(Mode::AbsoluteX, bus),
            0x1f => self.instr_slo(Mode::AbsoluteX, bus),
            0x20 => self.instr_jsr(Mode::Absolute, bus),
            0x21 => self.instr_and(Mode::XIndirect, bus),
            0x22 => self.instr_jam(),
            0x23 => self.instr_rla(Mode::XIndirect, bus),
            0x24 => self.instr_bit(Mode::ZeroPage, bus),
            0x25 => self.instr_and(Mode::ZeroPage, bus),
            0x26 => self.instr_rol(Mode::ZeroPage, bus),
            0x27 => self.instr_rla(Mode::ZeroPage, bus),
            0x28 => self.instr_plp(Mode::Implied, bus),
            0x29 => self.instr_and(Mode::Immediate, bus),
            0x2a => self.instr_rol(Mode::Accumulator, bus),
            0x2b => self.instr_anc(Mode::Immediate, bus),
            0x2c => self.instr_bit(Mode::Absolute, bus),
            0x2d => self.instr_and(Mode::Absolute, bus),
            0x2e => self.instr_rol(Mode::Absolute, bus),
            0x2f => self.instr_rla(Mode::Absolute, bus),
            0x30 => self.instr_bmi(Mode::Relative, bus),
            0x31 => self.instr_and(Mode::IndirectY, bus),
            0x32 => self.instr_jam(),
            0x33 => self.instr_rla(Mode::IndirectY, bus),
            0x34 => self.instr_nop_with_operand(Mode::ZeroPageX, bus),
            0x35 => self.instr_and(Mode::ZeroPageX, bus),
            0x36 => self.instr_rol(Mode::ZeroPageX, bus),
            0x37 => self.instr_rla(Mode::ZeroPageX, bus),
            0x38 => self.instr_sec(),
            0x39 => self.instr_and(Mode::AbsoluteY, bus),
            0x3a => self.instr_nop(),
            0x3b => self.instr_rla(Mode::AbsoluteY, bus),
            0x3c => self.instr_nop_with_operand(Mode::AbsoluteX, bus),
            0x3d => self.instr_and(Mode::AbsoluteX, bus),
            0x3e => self.instr_rol(Mode::AbsoluteX, bus),
            0x3f => self.instr_rla(Mode::AbsoluteX, bus),
            0x40 => self.instr_rti(bus),
            0x41 => self.instr_eor(Mode::XIndirect, bus),
            0x42 => self.instr_jam(),
            0x43 => self.instr_sre(Mode::XIndirect, bus),
            0x44 => self.instr_nop_with_operand(Mode::ZeroPage, bus),
            0x45 => self.instr_eor(Mode::ZeroPage, bus),
            0x46 => self.instr_lsr(Mode::ZeroPage, bus),
            0x47 => self.instr_sre(Mode::ZeroPage, bus),
            0x48 => self.instr_pha(Mode::Implied, bus),
            0x49 => self.instr_eor(Mode::Immediate, bus),
            0x4a => self.instr_lsr(Mode::Accumulator, bus),
            0x4b => self.instr_alr(Mode::Immediate, bus),
            0x4c => self.instr_jmp(Mode::Absolute, bus),
            0x4d => self.instr_eor(Mode::Absolute, bus),
            0x4e => self.instr_lsr(Mode::Absolute, bus),
            0x4f => self.instr_sre(Mode::Absolute, bus),
            0x50 => self.instr_bvc(Mode::Relative, bus),
            0x51 => self.instr_eor(Mode::IndirectY, bus),
            0x52 => self.instr_jam(),
            0x53 => self.instr_sre(Mode::IndirectY, bus),
            0x54 => self.instr_nop_with_operand(Mode::ZeroPageX, bus),
            0x55 => self.instr_eor(Mode::ZeroPageX, bus),
            0x56 => self.instr_lsr(Mode::ZeroPageX, bus),
            0x57 => self.instr_sre(Mode::ZeroPageX, bus),
            0x58 => self.instr_cli(),
            0x59 => self.instr_eor(Mode::AbsoluteY, bus),
            0x5a => self.instr_nop(),
            0x5b => self.instr_sre(Mode::AbsoluteY, bus),
            0x5c => self.instr_nop_with_operand(Mode::AbsoluteX, bus),
            0x5d => self.instr_eor(Mode::AbsoluteX, bus),
            0x5e => self.instr_lsr(Mode::AbsoluteX, bus),
            0x5f => self.instr_sre(Mode::AbsoluteX, bus),
            0x60 => self.instr_rts(Mode::Implied, bus),
            0x61 => self.instr_adc(Mode::XIndirect, bus),
            0x62 => self.instr_jam(),
            0x63 => self.instr_rra(Mode::XIndirect, bus),
            0x64 => self.instr_nop_with_operand(Mode::ZeroPage, bus),
            0x65 => self.instr_adc(Mode::ZeroPage, bus),
            0x66 => self.instr_ror(Mode::ZeroPage, bus),
            0x67 => self.instr_rra(Mode::ZeroPage, bus),
            0x68 => self.instr_pla(Mode::Implied, bus),
            0x69 => self.instr_adc(Mode::Immediate, bus),
            0x6a => self.instr_ror(Mode::Accumulator, bus),
            0x6b => self.instr_arr(Mode::Immediate, bus),
            0x6c => self.instr_jmp(Mode::Indirect, bus),
            0x6d => self.instr_adc(Mode::Absolute, bus),
            0x6e => self.instr_ror(Mode::Absolute, bus),
            0x6f => self.instr_rra(Mode::Absolute, bus),
            0x70 => self.instr_bvs(Mode::Relative, bus),
            0x71 => self.instr_adc(Mode::IndirectY, bus),
            0x72 => self.instr_jam(),
            0x73 => self.instr_rra(Mode::IndirectY, bus),
            0x74 => self.instr_nop_with_operand(Mode::ZeroPageX, bus),
            0x75 => self.instr_adc(Mode::ZeroPageX, bus),
            0x76 => self.instr_ror(Mode::ZeroPageX, bus),
            0x77 => self.instr_rra(Mode::ZeroPageX, bus),
            0x78 => self.instr_sei(),
            0x79 => self.instr_adc(Mode::AbsoluteY, bus),
            0x7a => self.instr_nop(),
            0x7b => self.instr_rra(Mode::AbsoluteY, bus),
            0x7c => self.instr_nop_with_operand(Mode::AbsoluteX, bus),
            0x7d => self.instr_adc(Mode::AbsoluteX, bus),
            0x7e => self.instr_ror(Mode::AbsoluteX, bus),
            0x7f => self.instr_rra(Mode::AbsoluteX, bus),
            0x80 => self.instr_nop_with_operand(Mode::Immediate, bus),
            0x81 => self.instr_sta(Mode::XIndirect, bus),
            0x82 => self.instr_nop_with_operand(Mode::Immediate, bus),
            0x83 => self.instr_sax(Mode::XIndirect, bus),
            0x84 => self.instr_sty(Mode::ZeroPage, bus),
            0x85 => self.instr_sta(Mode::ZeroPage, bus),
            0x86 => self.instr_stx(Mode::ZeroPage, bus),
            0x87 => self.instr_sax(Mode::ZeroPage, bus),
            0x88 => self.instr_dey(Mode::Implied, bus),
            0x89 => self.instr_nop_with_operand(Mode::Immediate, bus),
            0x8a => self.instr_txa(Mode::Implied, bus),
            0x8b => self.instr_ane(Mode::Immediate, bus),
            0x8c => self.instr_sty(Mode::Absolute, bus),
            0x8d => self.instr_sta(Mode::Absolute, bus),
            0x8e => self.instr_stx(Mode::Absolute, bus),
            0x8f => self.instr_sax(Mode::Absolute, bus),
            0x90 => self.instr_bcc(Mode::Relative, bus),
            0x91 => self.instr_sta(Mode::IndirectY, bus),
            0x92 => self.instr_jam(),
            0x93 => self.instr_sha(Mode::IndirectY, bus),
            0x94 => self.instr_sty(Mode::ZeroPageX, bus),
            0x95 => self.instr_sta(Mode::ZeroPageX, bus),
            0x96 => self.instr_stx(Mode::ZeroPageY, bus),
            0x97 => self.instr_sax(Mode::ZeroPageY, bus),
            0x98 => self.instr_tya(Mode::Implied, bus),
            0x99 => self.instr_sta(Mode::AbsoluteY, bus),
            0x9a => self.instr_txs(Mode::Implied, bus),
            0x9b => self.instr_tas(Mode::AbsoluteY, bus),
            0x9c => self.instr_shy(Mode::AbsoluteX, bus),
            0x9d => self.instr_sta(Mode::AbsoluteX, bus),
            0x9e => self.instr_shx(Mode::AbsoluteY, bus),
            0x9f => self.instr_sha(Mode::AbsoluteY, bus),
            0xa0 => self.instr_ldy(Mode::Immediate, bus),
            0xa1 => self.instr_lda(Mode::XIndirect, bus),
            0xa2 => self.instr_ldx(Mode::Immediate, bus),
            0xa3 => self.instr_lax(Mode::XIndirect, bus),
            0xa4 => self.instr_ldy(Mode::ZeroPage, bus),
            0xa5 => self.instr_lda(Mode::ZeroPage, bus),
            0xa6 => self.instr_ldx(Mode::ZeroPage, bus),
            0xa7 => self.instr_lax(Mode::ZeroPage, bus),
            0xa8 => self.instr_tay(Mode::Implied, bus),
            0xa9 => self.instr_lda(Mode::Immediate, bus),
            0xaa => self.instr_tax(Mode::Implied, bus),
            0xab => self.instr_lxa(Mode::Immediate, bus),
            0xac => self.instr_ldy(Mode::Absolute, bus),
            0xad => self.instr_lda(Mode::Absolute, bus),
            0xae => self.instr_ldx(Mode::Absolute, bus),
            0xaf => self.instr_lax(Mode::Absolute, bus),
            0xb0 => self.instr_bcs(Mode::Relative, bus),
            0xb1 => self.instr_lda(Mode::IndirectY, bus),
            0xb2 => self.instr_jam(),
            0xb3 => self.instr_lax(Mode::IndirectY, bus),
            0xb4 => self.instr_ldy(Mode::ZeroPageX, bus),
            0xb5 => self.instr_lda(Mode::ZeroPageX, bus),
            0xb6 => self.instr_ldx(Mode::ZeroPageY, bus),
            0xb7 => self.instr_lax(Mode::ZeroPageY, bus),
            0xb8 => self.instr_clv(),
            0xb9 => self.instr_lda(Mode::AbsoluteY, bus),
            0xba => self.instr_tsx(Mode::Implied, bus),
            0xbb => self.instr_las(Mode::AbsoluteY, bus),
            0xbc => self.instr_ldy(Mode::AbsoluteX, bus),
            0xbd => self.instr_lda(Mode::AbsoluteX, bus),
            0xbe => self.instr_ldx(Mode::AbsoluteY, bus),
            0xbf => self.instr_lax(Mode::AbsoluteY, bus),
            0xc0 => self.instr_cpy(Mode::Immediate, bus),
            0xc1 => self.instr_cmp(Mode::XIndirect, bus),
            0xc2 => self.instr_nop_with_operand(Mode::Immediate, bus),
            0xc3 => self.instr_dcp(Mode::XIndirect, bus),
            0xc4 => self.instr_cpy(Mode::ZeroPage, bus),
            0xc5 => self.instr_cmp(Mode::ZeroPage, bus),
            0xc6 => self.instr_dec(Mode::ZeroPage, bus),
            0xc7 => self.instr_dcp(Mode::ZeroPage, bus),
            0xc8 => self.instr_iny(Mode::Implied, bus),
            0xc9 => self.instr_cmp(Mode::Immediate, bus),
            0xca => self.instr_dex(Mode::Implied, bus),
            0xcb => self.instr_sbx(Mode::Immediate, bus),
            0xcc => self.instr_cpy(Mode::Absolute, bus),
            0xcd => self.instr_cmp(Mode::Absolute, bus),
            0xce => self.instr_dec(Mode::Absolute, bus),
            0xcf => self.instr_dcp(Mode::Absolute, bus),
            0xd0 => self.instr_bne(Mode::Relative, bus),
            0xd1 => self.instr_cmp(Mode::IndirectY, bus),
            0xd2 => self.instr_jam(),
            0xd3 => self.instr_dcp(Mode::IndirectY, bus),
            0xd4 => self.instr_nop_with_operand(Mode::ZeroPageX, bus),
            0xd5 => self.instr_cmp(Mode::ZeroPageX, bus),
            0xd6 => self.instr_dec(Mode::ZeroPageX, bus),
            0xd7 => self.instr_dcp(Mode::ZeroPageX, bus),
            0xd8 => self.instr_cld(),
            0xd9 => self.instr_cmp(Mode::AbsoluteY, bus),
            0xda => self.instr_nop(),
            0xdb => self.instr_dcp(Mode::AbsoluteY, bus),
            0xdc => self.instr_nop_with_operand(Mode::AbsoluteX, bus),
            0xdd => self.instr_cmp(Mode::AbsoluteX, bus),
            0xde => self.instr_dec(Mode::AbsoluteX, bus),
            0xdf => self.instr_dcp(Mode::AbsoluteX, bus),
            0xe0 => self.instr_cpx(Mode::Immediate, bus),
            0xe1 => self.instr_sbc(Mode::XIndirect, bus),
            0xe2 => self.instr_nop_with_operand(Mode::Immediate, bus),
            0xe3 => self.instr_isc(Mode::XIndirect, bus),
            0xe4 => self.instr_cpx(Mode::ZeroPage, bus),
            0xe5 => self.instr_sbc(Mode::ZeroPage, bus),
            0xe6 => self.instr_inc(Mode::ZeroPage, bus),
            0xe7 => self.instr_isc(Mode::ZeroPage, bus),
            0xe8 => self.instr_inx(Mode::Implied, bus),
            0xe9 => self.instr_sbc(Mode::Immediate, bus),
            0xea => self.instr_nop(),
            0xeb => self.instr_usbc(Mode::Immediate, bus),
            0xec => self.instr_cpx(Mode::Absolute, bus),
            0xed => self.instr_sbc(Mode::Absolute, bus),
            0xee => self.instr_inc(Mode::Absolute, bus),
            0xef => self.instr_isc(Mode::Absolute, bus),
            0xf0 => self.instr_beq(Mode::Relative, bus),
            0xf1 => self.instr_sbc(Mode::IndirectY, bus),
            0xf2 => self.instr_jam(),
            0xf3 => self.instr_isc(Mode::IndirectY, bus),
            0xf4 => self.instr_nop_with_operand(Mode::ZeroPageX, bus),
            0xf5 => self.instr_sbc(Mode::ZeroPageX, bus),
            0xf6 => self.instr_inc(Mode::ZeroPageX, bus),
            0xf7 => self.instr_isc(Mode::ZeroPageX, bus),
            0xf8 => self.instr_sed(),
            0xf9 => self.instr_sbc(Mode::AbsoluteY, bus),
            0xfa => self.instr_nop(),
            0xfb => self.instr_isc(Mode::AbsoluteY, bus),
            0xfc => self.instr_nop_with_operand(Mode::AbsoluteX, bus),
            0xfd => self.instr_sbc(Mode::AbsoluteX, bus),
            0xfe => self.instr_inc(Mode::AbsoluteX, bus),
            0xff => self.instr_isc(Mode::AbsoluteX, bus),
        }
    }
}