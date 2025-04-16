use OpCode::*;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]

pub enum OpCode {
    /// Illegal, not yet built
    Illegal(u8),
    Jam,

    AdcAbs,
    AdcAbsX,
    AdcAbsY,
    AdcImm,
    AdcIndY,
    AdcXInd,
    AdcZpg,
    AdcZpgX,
    AndAbs,
    AndAbsX,
    AndAbsY,
    AndImm,
    AndIndY,
    AndXInd,
    AndZpg,
    AndZpgX,
    AslA,
    AslAbs,
    AslAbsX,
    AslZpg,
    AslZpgX,
    BccRel,
    BcsRel,
    BeqRel,
    BitAbs,
    BitZpg,
    BmiRel,
    BneRel,
    BplRel,
    BrkImpl,
    BvcRel,
    BvsRel,
    ClcImpl,
    CldImpl,
    CliImpl,
    ClvImpl,
    CmpAbs,
    CmpAbsX,
    CmpAbsY,
    CmpImm,
    CmpIndY,
    CmpXInd,
    CmpZpg,
    CmpZpgX,
    CpxAbs,
    CpxImm,
    CpxZpg,
    CpyAbs,
    CpyImm,
    CpyZpg,
    DcpAbs,
    DcpAbsX,
    DcpAbsY,
    DcpXInd,
    DcpIndY,
    DcpZpg,
    DcpZpgX,
    DecAbs,
    DecAbsX,
    DecZpg,
    DecZpgX,
    DexImpl,
    DeyImpl,
    EorAbs,
    EorAbsX,
    EorAbsY,
    EorImm,
    EorIndY,
    EorXInd,
    EorZpg,
    EorZpgX,
    IncAbs,
    IncAbsX,
    IncZpg,
    IncZpgX,
    InxImpl,
    InyImpl,
    JmpAbs,
    JmpInd,
    JsrAbs,
    LaxAbs,
    LaxAbsY,
    LaxXInd,
    LaxIndY,
    LaxZpg,
    LaxZpgY,
    LdaAbs,
    LdaAbsX,
    LdaAbsY,
    LdaImm,
    LdaIndY,
    LdaXInd,
    LdaZpg,
    LdaZpgX,
    LdxAbs,
    LdxAbsY,
    LdxImm,
    LdxZpg,
    LdxZpgY,
    LdyAbs,
    LdyAbsX,
    LdyImm,
    LdyZpg,
    LdyZpgX,
    LsrA,
    LsrAbs,
    LsrAbsX,
    LsrZpg,
    LsrZpgX,
    NopAbs,
    NopAbsX,
    NopImm,
    NopImpl,
    NopZpg,
    NopZpgX,
    OraAbs,
    OraAbsX,
    OraAbsY,
    OraImm,
    OraIndY,
    OraXInd,
    OraZpg,
    OraZpgX,
    PhaImpl,
    PhpImpl,
    PlaImpl,
    PlpImpl,
    RolA,
    RolAbs,
    RolAbsX,
    RolZpg,
    RolZpgX,
    RorA,
    RorAbs,
    RorAbsX,
    RorZpg,
    RorZpgX,
    RtiImpl,
    RtsImpl,
    SaxAbs,
    SaxXind,
    SaxZpg,
    SaxZpgY,
    SbcAbs,
    SbcAbsX,
    SbcAbsY,
    SbcImm,
    SbcIndY,
    SbcXInd,
    SbcZpg,
    SbcZpgX,
    SecImpl,
    SedImpl,
    SeiImpl,
    StaAbs,
    StaAbsX,
    StaAbsY,
    StaIndY,
    StaXInd,
    StaZpg,
    StaZpgX,
    StxAbs,
    StxZpg,
    StxZpgY,
    StyAbs,
    StyZpg,
    StyZpgX,
    TaxImpl,
    TayImpl,
    TsxImpl,
    TxaImpl,
    TxsImpl,
    TyaImpl,
}

impl From<u8> for OpCode {
    fn from(value: u8) -> Self {
        match value {
            0x00 => BrkImpl,
            0x01 => OraXInd,
            0x02 => Jam,
            0x03 => Illegal(value),
            0x04 => NopZpg,
            0x05 => OraZpg,
            0x06 => AslZpg,
            0x07 => Illegal(value),
            0x08 => PhpImpl,
            0x09 => OraImm,
            0x0a => AslA,
            0x0b => Illegal(value),
            0x0c => NopAbs,
            0x0d => OraAbs,
            0x0e => AslAbs,
            0x0f => Illegal(value),

            0x10 => BplRel,
            0x11 => OraIndY,
            0x12 => Jam,
            0x13 => Illegal(value),
            0x14 => NopZpgX,
            0x15 => OraZpgX,
            0x16 => AslZpgX,
            0x17 => Illegal(value),
            0x18 => ClcImpl,
            0x19 => OraAbsY,
            0x1a => NopImpl,
            0x1b => Illegal(value),
            0x1c => NopAbsX,
            0x1d => OraAbsX,
            0x1e => AslAbsX,
            0x1f => Illegal(value),

            0x20 => JsrAbs,
            0x21 => AndXInd,
            0x22 => Jam,
            0x23 => Illegal(value),
            0x24 => BitZpg,
            0x25 => AndZpg,
            0x26 => RolZpg,
            0x27 => Illegal(value),
            0x28 => PlpImpl,
            0x29 => AndImm,
            0x2a => RolA,
            0x2b => Illegal(value),
            0x2c => BitAbs,
            0x2d => AndAbs,
            0x2e => RolAbs,
            0x2f => Illegal(value),

            0x30 => BmiRel,
            0x31 => AndIndY,
            0x32 => Jam,
            0x33 => Illegal(value),
            0x34 => NopZpgX,
            0x35 => AndZpgX,
            0x36 => RolZpgX,
            0x37 => Illegal(value),
            0x38 => SecImpl,
            0x39 => AndAbsY,
            0x3a => NopImpl,
            0x3b => Illegal(value),
            0x3c => NopAbsX,
            0x3d => AndAbsX,
            0x3e => RolAbsX,
            0x3f => Illegal(value),

            0x40 => RtiImpl,
            0x41 => EorXInd,
            0x42 => Jam,
            0x43 => Illegal(value),
            0x44 => NopZpg,
            0x45 => EorZpg,
            0x46 => LsrZpg,
            0x47 => Illegal(value),
            0x48 => PhaImpl,
            0x49 => EorImm,
            0x4a => LsrA,
            0x4b => Illegal(value),
            0x4c => JmpAbs,
            0x4d => EorAbs,
            0x4e => LsrAbs,
            0x4f => Illegal(value),

            0x50 => BvcRel,
            0x51 => EorIndY,
            0x52 => Jam,
            0x53 => Illegal(value),
            0x54 => NopZpgX,
            0x55 => EorZpgX,
            0x56 => LsrZpgX,
            0x57 => Illegal(value),
            0x58 => CliImpl,
            0x59 => EorAbsY,
            0x5a => NopImpl,
            0x5b => Illegal(value),
            0x5c => NopAbsX,
            0x5d => EorAbsX,
            0x5e => LsrAbsX,
            0x5f => Illegal(value),

            0x60 => RtsImpl,
            0x61 => AdcXInd,
            0x62 => Jam,
            0x63 => Illegal(value),
            0x64 => NopZpg,
            0x65 => AdcZpg,
            0x66 => RorZpg,
            0x67 => Illegal(value),
            0x68 => PlaImpl,
            0x69 => AdcImm,
            0x6a => RorA,
            0x6b => Illegal(value),
            0x6c => JmpInd,
            0x6d => AdcAbs,
            0x6e => RorAbs,
            0x6f => Illegal(value),

            0x70 => BvsRel,
            0x71 => AdcIndY,
            0x72 => Jam,
            0x73 => Illegal(value),
            0x74 => NopZpgX,
            0x75 => AdcZpgX,
            0x76 => RorZpgX,
            0x77 => Illegal(value),
            0x78 => SeiImpl,
            0x79 => AdcAbsY,
            0x7a => NopImpl,
            0x7b => Illegal(value),
            0x7c => NopAbsX,
            0x7d => AdcAbsX,
            0x7e => RorAbsX,
            0x7f => Illegal(value),

            0x80 => NopImm,
            0x81 => StaXInd,
            0x82 => NopImm,
            0x83 => SaxXind,
            0x84 => StyZpg,
            0x85 => StaZpg,
            0x86 => StxZpg,
            0x87 => SaxZpg,
            0x88 => DeyImpl,
            0x89 => NopImm,
            0x8a => TxaImpl,
            0x8b => Illegal(value),
            0x8c => StyAbs,
            0x8d => StaAbs,
            0x8e => StxAbs,
            0x8f => SaxAbs,

            0x90 => BccRel,
            0x91 => StaIndY,
            0x92 => Jam,
            0x93 => Illegal(value),
            0x94 => StyZpgX,
            0x95 => StaZpgX,
            0x96 => StxZpgY,
            0x97 => SaxZpgY,
            0x98 => TyaImpl,
            0x99 => StaAbsY,
            0x9a => TxsImpl,
            0x9b => Illegal(value),
            0x9c => Illegal(value),
            0x9d => StaAbsX,
            0x9e => Illegal(value),
            0x9f => Illegal(value),

            0xa0 => LdyImm,
            0xa1 => LdaXInd,
            0xa2 => LdxImm,
            0xa3 => LaxXInd,
            0xa4 => LdyZpg,
            0xa5 => LdaZpg,
            0xa6 => LdxZpg,
            0xa7 => LaxZpg,
            0xa8 => TayImpl,
            0xa9 => LdaImm,
            0xaa => TaxImpl,
            0xab => Illegal(value),
            0xac => LdyAbs,
            0xad => LdaAbs,
            0xae => LdxAbs,
            0xaf => LaxAbs,

            0xb0 => BcsRel,
            0xb1 => LdaIndY,
            0xb2 => Jam,
            0xb3 => LaxIndY,
            0xb4 => LdyZpgX,
            0xb5 => LdaZpgX,
            0xb6 => LdxZpgY,
            0xb7 => LaxZpgY,
            0xb8 => ClvImpl,
            0xb9 => LdaAbsY,
            0xba => TsxImpl,
            0xbb => Illegal(value),
            0xbc => LdyAbsX,
            0xbd => LdaAbsX,
            0xbe => LdxAbsY,
            0xbf => LaxAbsY,

            0xc0 => CpyImm,
            0xc1 => CmpXInd,
            0xc2 => NopImm,
            0xc3 => DcpXInd,
            0xc4 => CpyZpg,
            0xc5 => CmpZpg,
            0xc6 => DecZpg,
            0xc7 => DcpZpg,
            0xc8 => InyImpl,
            0xc9 => CmpImm,
            0xca => DexImpl,
            0xcb => Illegal(value),
            0xcc => CpyAbs,
            0xcd => CmpAbs,
            0xce => DecAbs,
            0xcf => DcpAbs,

            0xd0 => BneRel,
            0xd1 => CmpIndY,
            0xd2 => Jam,
            0xd3 => DcpIndY,
            0xd4 => NopZpgX,
            0xd5 => CmpZpgX,
            0xd6 => DecZpgX,
            0xd7 => DcpZpgX,
            0xd8 => CldImpl,
            0xd9 => CmpAbsY,
            0xda => NopImpl,
            0xdb => DcpAbsY,
            0xdc => NopAbsX,
            0xdd => CmpAbsX,
            0xde => DecAbsX,
            0xdf => DcpAbsX,

            0xe0 => CpxImm,
            0xe1 => SbcXInd,
            0xe2 => NopImm,
            0xe3 => Illegal(value),
            0xe4 => CpxZpg,
            0xe5 => SbcZpg,
            0xe6 => IncZpg,
            0xe7 => Illegal(value),
            0xe8 => InxImpl,
            0xe9 => SbcImm,
            0xea => NopImpl,
            0xeb => SbcImm, // TODO
            0xec => CpxAbs,
            0xed => SbcAbs,
            0xee => IncAbs,
            0xef => Illegal(value),

            0xf0 => BeqRel,
            0xf1 => SbcIndY,
            0xf2 => Jam,
            0xf3 => Illegal(value),
            0xf4 => NopZpgX,
            0xf5 => SbcZpgX,
            0xf6 => IncZpgX,
            0xf7 => Illegal(value),
            0xf8 => SedImpl,
            0xf9 => SbcAbsY,
            0xfa => NopImpl,
            0xfb => Illegal(value),
            0xfc => NopAbsX,
            0xfd => SbcAbsX,
            0xfe => IncAbsX,
            0xff => Illegal(value),
        }
    }
}

impl OpCode {
    pub const fn is_illegal(&self) -> bool {
        matches!(self, Illegal(_) | Jam)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_match_io_lda() {
        assert_eq!(LdaXInd, OpCode::from(0xa1));
        assert_eq!(LdaZpg, OpCode::from(0xa5));
        assert_eq!(LdaImm, OpCode::from(0xa9));
        assert_eq!(LdaAbs, OpCode::from(0xad));
        assert_eq!(LdaIndY, OpCode::from(0xb1));
        assert_eq!(LdaZpgX, OpCode::from(0xb5));
        assert_eq!(LdaAbsY, OpCode::from(0xb9));
        assert_eq!(LdaAbsX, OpCode::from(0xbd));
    }

    #[test]
    fn test_match_io_ldx() {
        assert_eq!(LdxImm, OpCode::from(0xa2));
        assert_eq!(LdxZpg, OpCode::from(0xa6));
        assert_eq!(LdxAbs, OpCode::from(0xae));
        assert_eq!(LdxZpgY, OpCode::from(0xb6));
        assert_eq!(LdxAbsY, OpCode::from(0xbe));
    }

    #[test]
    fn test_match_io_ldy() {
        assert_eq!(LdyImm, OpCode::from(0xa0));
        assert_eq!(LdyZpg, OpCode::from(0xa4));
        assert_eq!(LdyAbs, OpCode::from(0xac));
        assert_eq!(LdyZpgX, OpCode::from(0xb4));
        assert_eq!(LdyAbsX, OpCode::from(0xbc));
    }

    #[test]
    fn test_match_arithmetic_adc() {
        assert_eq!(AdcImm, OpCode::from(0x69));
        assert_eq!(AdcZpg, OpCode::from(0x65));
        assert_eq!(AdcZpgX, OpCode::from(0x75));
        assert_eq!(AdcAbs, OpCode::from(0x6D));
        assert_eq!(AdcAbsX, OpCode::from(0x7D));
        assert_eq!(AdcAbsY, OpCode::from(0x79));
        assert_eq!(AdcXInd, OpCode::from(0x61));
        assert_eq!(AdcIndY, OpCode::from(0x71));
    }

    #[test]
    fn test_match_arithmetic_sbc() {
        assert_eq!(SbcImm, OpCode::from(0xe9));
        assert_eq!(SbcZpg, OpCode::from(0xe5));
        assert_eq!(SbcZpgX, OpCode::from(0xf5));
        assert_eq!(SbcAbs, OpCode::from(0xeD));
        assert_eq!(SbcAbsX, OpCode::from(0xfD));
        assert_eq!(SbcAbsY, OpCode::from(0xf9));
        assert_eq!(SbcXInd, OpCode::from(0xe1));
        assert_eq!(SbcIndY, OpCode::from(0xf1));
    }

    #[test]
    fn test_match_arithmetic_increment() {
        assert_eq!(IncZpg, OpCode::from(0xe6));
        assert_eq!(IncZpgX, OpCode::from(0xf6));
        assert_eq!(IncAbs, OpCode::from(0xee));
        assert_eq!(IncAbsX, OpCode::from(0xfe));

        assert_eq!(InxImpl, OpCode::from(0xe8));
        assert_eq!(InyImpl, OpCode::from(0xc8));
    }

    #[test]
    fn test_match_arithmetic_decrement() {
        assert_eq!(DecZpg, OpCode::from(0xc6));
        assert_eq!(DecZpgX, OpCode::from(0xd6));
        assert_eq!(DecAbs, OpCode::from(0xce));
        assert_eq!(DecAbsX, OpCode::from(0xde));

        assert_eq!(DexImpl, OpCode::from(0xca));
        assert_eq!(DeyImpl, OpCode::from(0x88));
    }

    #[test]
    fn test_match_ill_jam() {
        assert_eq!(Jam, OpCode::from(0x02));
        assert_eq!(Jam, OpCode::from(0x12));
        assert_eq!(Jam, OpCode::from(0x22));
        assert_eq!(Jam, OpCode::from(0x32));
        assert_eq!(Jam, OpCode::from(0x42));
        assert_eq!(Jam, OpCode::from(0x52));
        assert_eq!(Jam, OpCode::from(0x62));
        assert_eq!(Jam, OpCode::from(0x72));

        assert_eq!(Jam, OpCode::from(0x92));

        assert_eq!(Jam, OpCode::from(0xb2));

        assert_eq!(Jam, OpCode::from(0xd2));

        assert_eq!(Jam, OpCode::from(0xf2));
    }

    #[test]
    fn test_match_ill_nops() {
        assert_eq!(NopImm, OpCode::from(0x80));
        assert_eq!(NopImm, OpCode::from(0x82));
        assert_eq!(NopImm, OpCode::from(0x89));
        assert_eq!(NopImm, OpCode::from(0xc2));
        assert_eq!(NopImm, OpCode::from(0xe2));

        assert_eq!(NopImpl, OpCode::from(0x1a));
        assert_eq!(NopImpl, OpCode::from(0x3a));
        assert_eq!(NopImpl, OpCode::from(0x5a));
        assert_eq!(NopImpl, OpCode::from(0x7a));
        assert_eq!(NopImpl, OpCode::from(0xda));
        assert_eq!(NopImpl, OpCode::from(0xea));
        assert_eq!(NopImpl, OpCode::from(0xfa));

        assert_eq!(NopAbs, OpCode::from(0x0c));

        assert_eq!(NopAbsX, OpCode::from(0x1c));
        assert_eq!(NopAbsX, OpCode::from(0x3c));
        assert_eq!(NopAbsX, OpCode::from(0x5c));
        assert_eq!(NopAbsX, OpCode::from(0x7c));
        assert_eq!(NopAbsX, OpCode::from(0xdc));
        assert_eq!(NopAbsX, OpCode::from(0xfc));

        assert_eq!(NopZpg, OpCode::from(0x04));
        assert_eq!(NopZpg, OpCode::from(0x44));
        assert_eq!(NopZpg, OpCode::from(0x64));

        assert_eq!(NopZpgX, OpCode::from(0x14));
        assert_eq!(NopZpgX, OpCode::from(0x34));
        assert_eq!(NopZpgX, OpCode::from(0x54));
        assert_eq!(NopZpgX, OpCode::from(0x74));
        assert_eq!(NopZpgX, OpCode::from(0xd4));
        assert_eq!(NopZpgX, OpCode::from(0xf4));
    }
}
