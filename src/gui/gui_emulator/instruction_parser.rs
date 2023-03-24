pub fn instruction_to_string(opcode: u8) -> Vec<String> {
    let mut operands: Vec<String> = Vec::new();

    match opcode {
        0x00 => {
            operands.push("BRK_impl".into());
        }
        0x01 => {
            operands.push("ORA_Xind".into());
            operands.push("$ Address LL".into());
        }
        0x02 => {
            operands.push("JAM".into());
        }
        0x03 => {
            operands.push("SLO_Xind".into());
            operands.push("$ Address LL".into());
        }
        0x04 => {
            operands.push("NOP_zpg".into());
            operands.push("$ Address LL".into());
        }
        0x05 => {
            operands.push("ORA_zpg".into());
            operands.push("$ Address LL".into());
        }
        0x06 => {
            operands.push("ASL_zpg".into());
            operands.push("$ Address LL".into());
        }
        0x07 => {
            operands.push("SLO_zpg".into());
            operands.push("$ Address LL".into());
        }
        0x08 => {
            operands.push("PHP_impl".into());
        }
        0x09 => {
            operands.push("ORA_imm".into());
            operands.push("$ Immediate Data".into());
        }
        0x0A => {
            operands.push("ASL_A".into());
        }
        0x0B => {
            operands.push("ANC_imm".into());
            operands.push("$ Immediate Data".into());
        }
        0x0C => {
            operands.push("NOP_abs".into());
            operands.push("$ Address LL".into());
            operands.push("$ Address HH".into());
        }
        0x0D => {
            operands.push("ORA_abs".into());
            operands.push("$ Address LL".into());
            operands.push("$ Address HH".into());
        }
        0x0E => {
            operands.push("ASL_abs".into());
            operands.push("$ Address LL".into());
            operands.push("$ Address HH".into());
        }
        0x0F => {
            operands.push("SLO_abs".into());
            operands.push("$ Address LL".into());
            operands.push("$ Address HH".into());
        }
        0x10 => {
            operands.push("BPL_rel".into());
            operands.push("$BB".into());
        }
        0x11 => {
            operands.push("ORA_indY".into());
            operands.push("$ Address LL".into());
        }
        0x12 => {
            operands.push("JAM".into());
        }
        0x13 => {
            operands.push("SLO_indY".into());
            operands.push("$ Address LL".into());
        }
        0x14 => {
            operands.push("NOP_zpgX".into());
            operands.push("$ Address LL".into());
        }
        0x15 => {
            operands.push("ORA_zpgX".into());
            operands.push("$ Address LL".into());
        }
        0x16 => {
            operands.push("ASL_zpgX".into());
            operands.push("$ Address LL".into());
        }
        0x17 => {
            operands.push("SLO_zpgX".into());
            operands.push("$ Address LL".into());
        }
        0x18 => {
            operands.push("CLC_impl".into());
        }
        0x19 => {
            operands.push("ORA_absY".into());
            operands.push("$ Address LL".into());
            operands.push("$ Address HH".into());
        }
        0x1A => {
            operands.push("NOP_impl".into());
        }
        0x1B => {
            operands.push("SLO_absY".into());
            operands.push("$ Address LL".into());
            operands.push("$ Address HH".into());
        }
        0x1C => {
            operands.push("NOP_absX".into());
            operands.push("$ Address LL".into());
            operands.push("$ Address HH".into());
        }
        0x1D => {
            operands.push("ORA_absX".into());
            operands.push("$ Address LL".into());
            operands.push("$ Address HH".into());
        }
        0x1E => {
            operands.push("ASL_absX".into());
            operands.push("$ Address LL".into());
            operands.push("$ Address HH".into());
        }
        0x1F => {
            operands.push("SLO_absX".into());
            operands.push("$ Address LL".into());
            operands.push("$ Address HH".into());
        }
        0x20 => {
            operands.push("JSR_abs".into());
            operands.push("$ Address LL".into());
            operands.push("$ Address HH".into());
        }
        0x21 => {
            operands.push("AND_Xind".into());
            operands.push("$ Address LL".into());
        }
        0x22 => {
            operands.push("JAM".into());
        }
        0x23 => {
            operands.push("RLA_Xind".into());
            operands.push("$ Address LL".into());
        }
        0x24 => {
            operands.push("BIT_zpg".into());
            operands.push("$ Address LL".into());
        }
        0x25 => {
            operands.push("AND_zpg".into());
            operands.push("$ Address LL".into());
        }
        0x26 => {
            operands.push("ROL_zpg".into());
            operands.push("$ Address LL".into());
        }
        0x27 => {
            operands.push("RLA_zpg".into());
            operands.push("$ Address LL".into());
        }
        0x28 => {
            operands.push("PLP_impl".into());
        }
        0x29 => {
            operands.push("AND_imm".into());
            operands.push("$ Immediate Data".into());
        }
        0x2A => {
            operands.push("ROL_A".into());
        }
        0x2B => {
            operands.push("ANC_imm".into());
            operands.push("$ Immediate Data".into());
        }
        0x2C => {
            operands.push("BIT_abs".into());
            operands.push("$ Address LL".into());
            operands.push("$ Address HH".into());
        }
        0x2D => {
            operands.push("AND_abs".into());
            operands.push("$ Address LL".into());
            operands.push("$ Address HH".into());
        }
        0x2E => {
            operands.push("ROL_abs".into());
            operands.push("$ Address LL".into());
            operands.push("$ Address HH".into());
        }
        0x2F => {
            operands.push("RLA_abs".into());
            operands.push("$ Address LL".into());
            operands.push("$ Address HH".into());
        }
        0x30 => {
            operands.push("BMI_rel".into());
            operands.push("$BB".into());
        }
        0x31 => {
            operands.push("AND_indY".into());
            operands.push("$ Address LL".into());
        }
        0x32 => {
            operands.push("JAM".into());
        }
        0x33 => {
            operands.push("RLA_indY".into());
            operands.push("$ Address LL".into());
        }
        0x34 => {
            operands.push("NOP_zpgX".into());
            operands.push("$ Address LL".into());
        }
        0x35 => {
            operands.push("AND_zpgX".into());
            operands.push("$ Address LL".into());
        }
        0x36 => {
            operands.push("ROL_zpgX".into());
            operands.push("$ Address LL".into());
        }
        0x37 => {
            operands.push("RLA_zpgX".into());
            operands.push("$ Address LL".into());
        }
        0x38 => {
            operands.push("SEC_impl".into());
        }
        0x39 => {
            operands.push("AND_absY".into());
            operands.push("$ Address LL".into());
            operands.push("$ Address HH".into());
        }
        0x3A => {
            operands.push("NOP_impl".into());
        }
        0x3B => {
            operands.push("RLA_absY".into());
            operands.push("$ Address LL".into());
            operands.push("$ Address HH".into());
        }
        0x3C => {
            operands.push("NOP_absX".into());
            operands.push("$ Address LL".into());
            operands.push("$ Address HH".into());
        }
        0x3D => {
            operands.push("AND_absX".into());
            operands.push("$ Address LL".into());
            operands.push("$ Address HH".into());
        }
        0x3E => {
            operands.push("ROL_absX".into());
            operands.push("$ Address LL".into());
            operands.push("$ Address HH".into());
        }
        0x3F => {
            operands.push("RLA_absX".into());
            operands.push("$ Address LL".into());
            operands.push("$ Address HH".into());
        }
        0x40 => {
            operands.push("RTI_impl".into());
        }
        0x41 => {
            operands.push("EOR_Xind".into());
            operands.push("$ Address LL".into());
        }
        0x42 => {
            operands.push("JAM".into());
        }
        0x43 => {
            operands.push("SRE_Xind".into());
            operands.push("$ Address LL".into());
        }
        0x44 => {
            operands.push("NOP_zpg".into());
            operands.push("$ Address LL".into());
        }
        0x45 => {
            operands.push("EOR_zpg".into());
            operands.push("$ Address LL".into());
        }
        0x46 => {
            operands.push("LSR_zpg".into());
            operands.push("$ Address LL".into());
        }
        0x47 => {
            operands.push("SRE_zpg".into());
            operands.push("$ Address LL".into());
        }
        0x48 => {
            operands.push("PHA_impl".into());
        }
        0x49 => {
            operands.push("EOR_imm".into());
            operands.push("$ Immediate Data".into());
        }
        0x4A => {
            operands.push("LSR_A".into());
        }
        0x4B => {
            operands.push("ALR_imm".into());
            operands.push("$ Immediate Data".into());
        }
        0x4C => {
            operands.push("JMP_abs".into());
            operands.push("$ Address LL".into());
            operands.push("$ Address HH".into());
        }
        0x4D => {
            operands.push("EOR_abs".into());
            operands.push("$ Address LL".into());
            operands.push("$ Address HH".into());
        }
        0x4E => {
            operands.push("LSR_abs".into());
            operands.push("$ Address LL".into());
            operands.push("$ Address HH".into());
        }
        0x4F => {
            operands.push("SRE_abs".into());
            operands.push("$ Address LL".into());
            operands.push("$ Address HH".into());
        }
        0x50 => {
            operands.push("BVC_rel".into());
            operands.push("$BB".into());
        }
        0x51 => {
            operands.push("EOR_indY".into());
            operands.push("$ Address LL".into());
        }
        0x52 => {
            operands.push("JAM".into());
        }
        0x53 => {
            operands.push("SRE_indY".into());
            operands.push("$ Address LL".into());
        }
        0x54 => {
            operands.push("NOP_zpgX".into());
            operands.push("$ Address LL".into());
        }
        0x55 => {
            operands.push("EOR_zpgX".into());
            operands.push("$ Address LL".into());
        }
        0x56 => {
            operands.push("LSR_zpgX".into());
            operands.push("$ Address LL".into());
        }
        0x57 => {
            operands.push("SRE_zpgX".into());
            operands.push("$ Address LL".into());
        }
        0x58 => {
            operands.push("CLI_impl".into());
        }
        0x59 => {
            operands.push("EOR_absY".into());
            operands.push("$ Address LL".into());
            operands.push("$ Address HH".into());
        }
        0x5A => {
            operands.push("NOP_impl".into());
        }
        0x5B => {
            operands.push("SRE_absY".into());
            operands.push("$ Address LL".into());
            operands.push("$ Address HH".into());
        }
        0x5C => {
            operands.push("NOP_absX".into());
            operands.push("$ Address LL".into());
            operands.push("$ Address HH".into());
        }
        0x5D => {
            operands.push("EOR_absX".into());
            operands.push("$ Address LL".into());
            operands.push("$ Address HH".into());
        }
        0x5E => {
            operands.push("LSR_absX".into());
            operands.push("$ Address LL".into());
            operands.push("$ Address HH".into());
        }
        0x5F => {
            operands.push("SRE_absX".into());
            operands.push("$ Address LL".into());
            operands.push("$ Address HH".into());
        }
        0x60 => {
            operands.push("RTS_impl".into());
        }
        0x61 => {
            operands.push("ADC_Xind".into());
            operands.push("$ Address LL".into());
        }
        0x62 => {
            operands.push("JAM".into());
        }
        0x63 => {
            operands.push("RRA_Xind".into());
            operands.push("$ Address LL".into());
        }
        0x64 => {
            operands.push("NOP_zpg".into());
            operands.push("$ Address LL".into());
        }
        0x65 => {
            operands.push("ADC_zpg".into());
            operands.push("$ Address LL".into());
        }
        0x66 => {
            operands.push("ROR_zpg".into());
            operands.push("$ Address LL".into());
        }
        0x67 => {
            operands.push("RRA_zpg".into());
            operands.push("$ Address LL".into());
        }
        0x68 => {
            operands.push("PLA_impl".into());
        }
        0x69 => {
            operands.push("ADC_imm".into());
            operands.push("$ Immediate Data".into());
        }
        0x6A => {
            operands.push("ROR_A".into());
        }
        0x6B => {
            operands.push("ARR_imm".into());
            operands.push("$ Immediate Data".into());
        }
        0x6C => {
            operands.push("JMP_ind".into());
            operands.push("$ Address LL".into());
            operands.push("$ Address HH".into());
        }
        0x6D => {
            operands.push("ADC_abs".into());

            operands.push("$ Address LL".into());
            operands.push("$ Address HH".into());
        }
        0x6E => {
            operands.push("ROR_abs".into());
            operands.push("$ Address LL".into());
            operands.push("$ Address HH".into());
        }
        0x6F => {
            operands.push("RRA_abs".into());
            operands.push("$ Address LL".into());
            operands.push("$ Address HH".into());
        }
        0x70 => {
            operands.push("BVS_rel".into());
            operands.push("$BB".into());
        }
        0x71 => {
            operands.push("ADC_indY".into());
            operands.push("$ Address LL".into());
        }
        0x72 => {
            operands.push("JAM".into());
        }
        0x73 => {
            operands.push("RRA_indY".into());
            operands.push("$ Address LL".into());
        }
        0x74 => {
            operands.push("NOP_zpgX".into());
            operands.push("$ Address LL".into());
        }
        0x75 => {
            operands.push("ADC_zpgX".into());
            operands.push("$ Address LL".into());
        }
        0x76 => {
            operands.push("ROR_zpgX".into());
            operands.push("$ Address LL".into());
        }
        0x77 => {
            operands.push("RRA_zpgX".into());
            operands.push("$ Address LL".into());
        }
        0x78 => {
            operands.push("SEI_impl".into());
        }
        0x79 => {
            operands.push("ADC_absY".into());
            operands.push("$ Address LL".into());
            operands.push("$ Address HH".into());
        }
        0x7a => {
            operands.push("NOP_impl".into());
        }
        0x7b => {
            operands.push("RRA_absY".into());
            operands.push("$ Address LL".into());
            operands.push("$ Address HH".into());
        }
        0x7c => {
            operands.push("NOP_absX".into());
            operands.push("$ Address LL".into());
            operands.push("$ Address HH".into());
        }
        0x7d => {
            operands.push("ADC_absX".into());
            operands.push("$ Address LL".into());
            operands.push("$ Address HH".into());
        }
        0x7e => {
            operands.push("ROR_absX".into());
            operands.push("$ Address LL".into());
            operands.push("$ Address HH".into());
        }
        0x7f => {
            operands.push("RRA_absX".into());
            operands.push("$ Address LL".into());
            operands.push("$ Address HH".into());
        }
        0x80 => {
            operands.push("NOP_imm".into());
            operands.push("$ Immediate Data".into());
        }
        0x81 => {
            operands.push("STA_Xind".into());
            operands.push("$ Address LL".into());
        }
        0x82 => {
            operands.push("NOP_imm".into());
            operands.push("$ Immediate Data".into());
        }
        0x83 => {
            operands.push("SAX_Xind".into());
            operands.push("$ Address LL".into());
        }
        0x84 => {
            operands.push("STY_zpg".into());
            operands.push("$ Address LL".into());
        }
        0x85 => {
            operands.push("STA_zpg".into());
            operands.push("$ Address LL".into());
        }
        0x86 => {
            operands.push("STX_zpg".into());
            operands.push("$ Address LL".into());
        }
        0x87 => {
            operands.push("SAX_zpg".into());
            operands.push("$ Address LL".into());
        }
        0x88 => {
            operands.push("DEY_impl".into());
        }
        0x89 => {
            operands.push("NOP_imm".into());
            operands.push("$ Immediate Data".into());
        }
        0x8a => {
            operands.push("TXA_impl".into());
        }
        0x8b => {
            operands.push("ANE_imm".into());
            operands.push("$ Immediate Data".into());
        }
        0x8c => {
            operands.push("STY_abs".into());
            operands.push("$ Address LL".into());
            operands.push("$ Address HH".into());
        }
        0x8d => {
            operands.push("STA_abs".into());
            operands.push("$ Address LL".into());
            operands.push("$ Address HH".into());
        }
        0x8e => {
            operands.push("STX_abs".into());
            operands.push("$ Address LL".into());
            operands.push("$ Address HH".into());
        }
        0x8f => {
            operands.push("SAX_abs".into());
            operands.push("$ Address LL".into());
            operands.push("$ Address HH".into());
        }
        0x90 => {
            operands.push("BCC_rel".into());
            operands.push("$BB".into());
        }
        0x91 => {
            operands.push("STA_indY".into());
            operands.push("$ Address LL".into());
        }
        0x92 => {
            operands.push("JAM".into());
        }
        0x93 => {
            operands.push("SHA_indY".into());
            operands.push("$ Address LL".into());
        }
        0x94 => {
            operands.push("STY_zpgX".into());
            operands.push("$ Address LL".into());
        }
        0x95 => {
            operands.push("STA_zpgX".into());
            operands.push("$ Address LL".into());
        }
        0x96 => {
            operands.push("STX_zpgY".into());
            operands.push("$ Address LL".into());
        }
        0x97 => {
            operands.push("SAX_zpgY".into());
            operands.push("$ Address LL".into());
        }
        0x98 => {
            operands.push("TYA_impl".into());
        }
        0x99 => {
            operands.push("STA_absY".into());
            operands.push("$ Address LL".into());
            operands.push("$ Address HH".into());
        }
        0x9a => {
            operands.push("TXS_impl".into());
        }
        0x9b => {
            operands.push("TAS_absY".into());
            operands.push("$ Address LL".into());
            operands.push("$ Address HH".into());
        }
        0x9c => {
            operands.push("SHY_absX".into());
            operands.push("$ Address LL".into());
            operands.push("$ Address HH".into());
        }
        0x9d => {
            operands.push("STA_absX".into());
            operands.push("$ Address LL".into());
            operands.push("$ Address HH".into());
        }
        0x9e => {
            operands.push("SHX_absY".into());
            operands.push("$ Address LL".into());
            operands.push("$ Address HH".into());
        }
        0x9f => {
            operands.push("SHA_absY".into());
            operands.push("$ Address LL".into());
            operands.push("$ Address HH".into());
        }
        0xa0 => {
            operands.push("LDY_imm".into());
            operands.push("$ Immediate Data".into());
        }
        0xa1 => {
            operands.push("LDA_Xind".into());
            operands.push("$ Address LL".into());
        }
        0xa2 => {
            operands.push("LDX_imm".into());
            operands.push("$ Immediate Data".into());
        }
        0xa3 => {
            operands.push("LAX_Xind".into());
            operands.push("$ Address LL".into());
        }
        0xa4 => {
            operands.push("LDY_zpg".into());
            operands.push("$ Address LL".into());
        }
        0xa5 => {
            operands.push("LDA_zpg".into());
            operands.push("$ Address LL".into());
        }
        0xa6 => {
            operands.push("LDX_zpg".into());
            operands.push("$ Address LL".into());
        }
        0xa7 => {
            operands.push("LAX_zpg".into());
            operands.push("$ Address LL".into());
        }
        0xa8 => {
            operands.push("TAY_impl".into());
        }
        0xa9 => {
            operands.push("LDA_imm".into());
            operands.push("$ Immediate Data".into());
        }
        0xaa => {
            operands.push("TAX_impl".into());
        }
        0xab => {
            operands.push("LXA_imm".into());
            operands.push("$ Immediate Data".into());
        }
        0xac => {
            operands.push("LDY_abs".into());
            operands.push("$ Address LL".into());
            operands.push("$ Address HH".into());
        }
        0xad => {
            operands.push("LDA_abs".into());
            operands.push("$ Address LL".into());
            operands.push("$ Address HH".into());
        }
        0xae => {
            operands.push("LDX_abs".into());
            operands.push("$ Address LL".into());
            operands.push("$ Address HH".into());
        }
        0xaf => {
            operands.push("LAX_abs".into());
            operands.push("$ Address LL".into());
            operands.push("$ Address HH".into());
        }
        0xb0 => {
            operands.push("BCS_rel".into());
            operands.push("$BB".into());
        }
        0xb1 => {
            operands.push("LDA_indY".into());
            operands.push("$ Address LL".into());
        }
        0xb2 => {
            operands.push("JAM".into());
        }
        0xb3 => {
            operands.push("LAX_indY".into());
            operands.push("$ Address LL".into());
        }
        0xb4 => {
            operands.push("LDY_zpgX".into());
            operands.push("$ Address LL".into());
        }
        0xb5 => {
            operands.push("LDA_zpgX".into());
            operands.push("$ Address LL".into());
        }
        0xb6 => {
            operands.push("LDX_zpgY".into());
            operands.push("$ Address LL".into());
        }
        0xb7 => {
            operands.push("LAX_zpgY".into());
            operands.push("$ Address LL".into());
        }
        0xb8 => {
            operands.push("CLV_impl".into());
        }
        0xb9 => {
            operands.push("LDA_absY".into());
            operands.push("$ Address LL".into());
            operands.push("$ Address HH".into());
        }
        0xba => {
            operands.push("TSX_impl".into());
        }
        0xbb => {
            operands.push("LAS_absY".into());
            operands.push("$ Address LL".into());
            operands.push("$ Address HH".into());
        }
        0xbc => {
            operands.push("LDY_absX".into());
            operands.push("$ Address LL".into());
            operands.push("$ Address HH".into());
        }
        0xbd => {
            operands.push("LDA_absX".into());
            operands.push("$ Address LL".into());
            operands.push("$ Address HH".into());
        }
        0xbe => {
            operands.push("LDX_absY".into());
            operands.push("$ Address LL".into());
            operands.push("$ Address HH".into());
        }
        0xbf => {
            operands.push("LAX_absY".into());
            operands.push("$ Address LL".into());
            operands.push("$ Address HH".into());
        }
        0xc0 => {
            operands.push("CPY_imm".into());
            operands.push("$ Immediate Data".into());
        }
        0xc1 => {
            operands.push("CMP_Xind".into());
            operands.push("$ Address LL".into());
        }
        0xc2 => {
            operands.push("NOP_imm".into());
            operands.push("$ Immediate Data".into());
        }
        0xc3 => {
            operands.push("DCP_Xind".into());
            operands.push("$ Address LL".into());
        }
        0xc4 => {
            operands.push("CPY_zpg".into());
            operands.push("$ Address LL".into());
        }
        0xc5 => {
            operands.push("CMP_zpg".into());
            operands.push("$ Address LL".into());
        }
        0xc6 => {
            operands.push("DEC_zpg".into());
            operands.push("$ Address LL".into());
        }
        0xc7 => {
            operands.push("DCP_zpg".into());
            operands.push("$ Address LL".into());
        }
        0xc8 => {
            operands.push("INY_impl".into());
        }
        0xc9 => {
            operands.push("CMP_imm".into());
            operands.push("$ Immediate Data".into());
        }
        0xca => {
            operands.push("DEX_impl".into());
        }
        0xcb => {
            operands.push("SBX_imm".into());
            operands.push("$ Immediate Data".into());
        }
        0xcc => {
            operands.push("CPY_abs".into());
            operands.push("$ Address LL".into());
            operands.push("$ Address HH".into());
        }
        0xcd => {
            operands.push("CMP_abs".into());
            operands.push("$ Address LL".into());
            operands.push("$ Address HH".into());
        }
        0xce => {
            operands.push("DEC_abs".into());
            operands.push("$ Address LL".into());
            operands.push("$ Address HH".into());
        }
        0xcf => {
            operands.push("DCP_abs".into());
            operands.push("$ Address LL".into());
            operands.push("$ Address HH".into());
        }
        0xd0 => {
            operands.push("BNE_rel".into());
            operands.push("$BB".into());
        }
        0xd1 => {
            operands.push("CMP_indY".into());
            operands.push("$ Address LL".into());
        }
        0xd2 => {
            operands.push("JAM".into());
        }
        0xd3 => {
            operands.push("DCP_indY".into());
            operands.push("$ Address LL".into());
        }
        0xd4 => {
            operands.push("NOP_zpgX".into());
            operands.push("$ Address LL".into());
        }
        0xd5 => {
            operands.push("CMP_zpgX".into());
            operands.push("$ Address LL".into());
        }
        0xd6 => {
            operands.push("DEC_zpgX".into());
            operands.push("$ Address LL".into());
        }
        0xd7 => {
            operands.push("DCP_zpgX".into());
            operands.push("$ Address LL".into());
        }
        0xd8 => {
            operands.push("CLD_impl".into());
        }
        0xd9 => {
            operands.push("CMP_absY".into());
            operands.push("$ Address LL".into());
            operands.push("$ Address HH".into());
        }
        0xda => {
            operands.push("NOP_impl".into());
        }
        0xdb => {
            operands.push("DCP_absY".into());
            operands.push("$ Address LL".into());
            operands.push("$ Address HH".into());
        }
        0xdc => {
            operands.push("NOP_absX".into());
            operands.push("$ Address LL".into());
            operands.push("$ Address HH".into());
        }
        0xdd => {
            operands.push("CMP_absX".into());
            operands.push("$ Address LL".into());
            operands.push("$ Address HH".into());
        }
        0xde => {
            operands.push("DEC_absX".into());
            operands.push("$ Address LL".into());
            operands.push("$ Address HH".into());
        }
        0xdf => {
            operands.push("DCP_absX".into());
            operands.push("$ Address LL".into());
            operands.push("$ Address HH".into());
        }
        0xe0 => {
            operands.push("CPX_imm".into());
            operands.push("$ Immediate Data".into());
        }
        0xe1 => {
            operands.push("SBC_Xind".into());
            operands.push("$ Address LL".into());
        }
        0xe2 => {
            operands.push("NOP_imm".into());
            operands.push("$ Immediate Data".into());
        }
        0xe3 => {
            operands.push("ISC_Xind".into());
            operands.push("$ Address LL".into());
        }
        0xe4 => {
            operands.push("CPX_zpg".into());
            operands.push("$ Address LL".into());
        }
        0xe5 => {
            operands.push("SBC_zpg".into());
            operands.push("$ Address LL".into());
        }
        0xe6 => {
            operands.push("INC_zpg".into());
            operands.push("$ Address LL".into());
        }
        0xe7 => {
            operands.push("ISC_zpg".into());
            operands.push("$ Address LL".into());
        }
        0xe8 => {
            operands.push("INX_impl".into());
        }
        0xe9 => {
            operands.push("SBC_imm".into());
            operands.push("$ Immediate Data".into());
        }
        0xea => {
            operands.push("NOP_impl".into());
        }
        0xeb => {
            operands.push("USBC_imm".into());
            operands.push("$ Immediate Data".into());
        }
        0xec => {
            operands.push("CPX_abs".into());
            operands.push("$ Address LL".into());
            operands.push("$ Address HH".into());
        }
        0xed => {
            operands.push("SBC_abs".into());
            operands.push("$ Address LL".into());
            operands.push("$ Address HH".into());
        }
        0xee => {
            operands.push("INC_abs".into());
            operands.push("$ Address LL".into());
            operands.push("$ Address HH".into());
        }
        0xef => {
            operands.push("ISC_abs".into());
            operands.push("$ Address LL".into());
            operands.push("$ Address HH".into());
        }
        0xf0 => {
            operands.push("BEQ_rel".into());
            operands.push("$BB".into());
        }
        0xf1 => {
            operands.push("SBC_indY".into());
            operands.push("$ Address LL".into());
        }
        0xf2 => {
            operands.push("JAM".into());
        }
        0xf3 => {
            operands.push("ISC_indY".into());
            operands.push("$ Address LL".into());
        }
        0xf4 => {
            operands.push("NOP_zpgX".into());
            operands.push("$ Address LL".into());
        }
        0xf5 => {
            operands.push("SBC_zpgX".into());
            operands.push("$ Address LL".into());
        }
        0xf6 => {
            operands.push("INC_zpgX".into());
            operands.push("$ Address LL".into());
        }
        0xf7 => {
            operands.push("ISC_zpgX".into());
            operands.push("$ Address LL".into());
        }
        0xf8 => {
            operands.push("SED_impl".into());
        }
        0xf9 => {
            operands.push("SBC_absY".into());
            operands.push("$ Address LL".into());
            operands.push("$ Address HH".into());
        }
        0xfa => {
            operands.push("NOP_impl".into());
        }
        0xfb => {
            operands.push("ISC_absY".into());
            operands.push("$ Address LL".into());
            operands.push("$ Address HH".into());
        }
        0xfc => {
            operands.push("NOP_absX".into());
            operands.push("$ Address LL".into());
            operands.push("$ Address HH".into());
        }
        0xfd => {
            operands.push("SBC_absX".into());
            operands.push("$ Address LL".into());
            operands.push("$ Address HH".into());
        }
        0xfe => {
            operands.push("INC_absX".into());
            operands.push("$ Address LL".into());
            operands.push("$ Address HH".into());
        }
        0xff => {
            operands.push("ISC_absX".into());
            operands.push("$ Address LL".into());
            operands.push("$ Address HH".into());
        }
    }

    operands
}
