use std::fmt::Debug;

use nesmc_emu::NesMachine;
use nesmc_types::instruction::OpCode;

pub enum Operand {
    Todo,

    A,
    Abs(u16),
    AbsX(u16),
    AbsY(u16),
    Imm(u8),
    Impl,
    Ind(u16),
    XInd(u8),
    IndY(u8),
    Rel(u8),
    Zpg(u8),
    ZpgX(u8),
    ZpgY(u8),
}

impl Debug for Operand {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Operand::Todo => write!(f, "Unknown, not yet implemented"),
            Operand::A => write!(f, "A"),
            Operand::Abs(val) => write!(f, "ZpgX {val:04X} "),
            Operand::AbsX(val) => write!(f, "AbsZ {val:02X} + X"),
            Operand::AbsY(val) => write!(f, "AbsY {val:02X} + Y"),
            Operand::Imm(val) => write!(f, "Imm {val:02X}"),
            Operand::Impl => write!(f, "Implied"),
            Operand::Ind(val) => write!(f, "Ind {val:04X}"),
            Operand::XInd(val) => write!(f, "XInd {val:02X} TODO"),
            Operand::IndY(val) => write!(f, "IndY {val:02X} TODO"),
            Operand::Rel(val) => write!(f, "Rel {val:02X}"),
            Operand::Zpg(val) => write!(f, "Zpg {val:02X}"),
            Operand::ZpgX(val) => write!(f, "ZpgX {val:02X} + X"),
            Operand::ZpgY(val) => write!(f, "ZpgY {val:02X} + Y"),
        }
    }
}

impl Operand {
    /// addr is the addr of opcode
    pub fn from_read_machine(op_code: OpCode, machine: &NesMachine, addr: u16) -> Self {
        match op_code {
            OpCode::Illegal(_) => Self::Todo,

            OpCode::AslA | OpCode::LsrA | OpCode::RolA | OpCode::RorA => Self::A,

            OpCode::AdcAbs
            | OpCode::AndAbs
            | OpCode::AslAbs
            | OpCode::BitAbs
            | OpCode::CmpAbs
            | OpCode::CpxAbs
            | OpCode::CpyAbs
            | OpCode::DecAbs
            | OpCode::EorAbs
            | OpCode::IncAbs
            | OpCode::JmpAbs
            | OpCode::JsrAbs
            | OpCode::LdaAbs
            | OpCode::LdxAbs
            | OpCode::LdyAbs
            | OpCode::LsrAbs
            | OpCode::OraAbs
            | OpCode::RolAbs
            | OpCode::RorAbs
            | OpCode::SbcAbs
            | OpCode::StaAbs
            | OpCode::StxAbs
            | OpCode::StyAbs
            | OpCode::NopAbs
            | OpCode::LaxAbs
            | OpCode::SaxAbs
            | OpCode::DcpAbs => {
                let l = machine.bus.read(addr.wrapping_add(1)) as u16;
                let h = machine.bus.read(addr.wrapping_add(2)) as u16;
                Self::Abs((h << 8) + l)
            }

            OpCode::AdcAbsX
            | OpCode::AndAbsX
            | OpCode::AslAbsX
            | OpCode::CmpAbsX
            | OpCode::DecAbsX
            | OpCode::EorAbsX
            | OpCode::IncAbsX
            | OpCode::LdaAbsX
            | OpCode::LdyAbsX
            | OpCode::LsrAbsX
            | OpCode::OraAbsX
            | OpCode::RolAbsX
            | OpCode::RorAbsX
            | OpCode::SbcAbsX
            | OpCode::StaAbsX
            | OpCode::NopAbsX
            | OpCode::DcpAbsX => {
                let l = machine.bus.read(addr.wrapping_add(1)) as u16;
                let h = machine.bus.read(addr.wrapping_add(2)) as u16;
                Self::AbsX((h << 8) + l)
            }

            OpCode::AdcAbsY
            | OpCode::AndAbsY
            | OpCode::CmpAbsY
            | OpCode::EorAbsY
            | OpCode::LdaAbsY
            | OpCode::LdxAbsY
            | OpCode::OraAbsY
            | OpCode::SbcAbsY
            | OpCode::StaAbsY
            | OpCode::LaxAbsY
            | OpCode::DcpAbsY => {
                let l = machine.bus.read(addr.wrapping_add(1)) as u16;
                let h = machine.bus.read(addr.wrapping_add(2)) as u16;
                Self::AbsY((h << 8) + l)
            }

            OpCode::AdcImm
            | OpCode::AndImm
            | OpCode::CmpImm
            | OpCode::CpxImm
            | OpCode::CpyImm
            | OpCode::EorImm
            | OpCode::LdaImm
            | OpCode::LdxImm
            | OpCode::LdyImm
            | OpCode::OraImm
            | OpCode::SbcImm
            | OpCode::NopImm => Self::Imm(machine.bus.read(addr.wrapping_add(1))),

            OpCode::Jam
            | OpCode::BrkImpl
            | OpCode::ClcImpl
            | OpCode::CldImpl
            | OpCode::CliImpl
            | OpCode::ClvImpl
            | OpCode::DexImpl
            | OpCode::DeyImpl
            | OpCode::InxImpl
            | OpCode::InyImpl
            | OpCode::NopImpl
            | OpCode::PhaImpl
            | OpCode::PhpImpl
            | OpCode::PlaImpl
            | OpCode::PlpImpl
            | OpCode::RtiImpl
            | OpCode::RtsImpl
            | OpCode::SecImpl
            | OpCode::SedImpl
            | OpCode::SeiImpl
            | OpCode::TaxImpl
            | OpCode::TayImpl
            | OpCode::TsxImpl
            | OpCode::TxaImpl
            | OpCode::TxsImpl
            | OpCode::TyaImpl => Self::Impl,

            OpCode::JmpInd => {
                let l = machine.bus.read(addr.wrapping_add(1)) as u16;
                let h = machine.bus.read(addr.wrapping_add(2)) as u16;
                Self::Ind((h << 8) + l)
            }

            OpCode::AdcXInd
            | OpCode::AndXInd
            | OpCode::CmpXInd
            | OpCode::EorXInd
            | OpCode::LdaXInd
            | OpCode::OraXInd
            | OpCode::SbcXInd
            | OpCode::StaXInd
            | OpCode::LaxXInd
            | OpCode::SaxXind
            | OpCode::DcpXInd => Self::XInd(machine.bus.read(addr.wrapping_add(1))),

            OpCode::AdcIndY
            | OpCode::AndIndY
            | OpCode::CmpIndY
            | OpCode::EorIndY
            | OpCode::LdaIndY
            | OpCode::OraIndY
            | OpCode::SbcIndY
            | OpCode::StaIndY
            | OpCode::LaxIndY
            | OpCode::DcpIndY => Self::IndY(machine.bus.read(addr.wrapping_add(1))),

            OpCode::BccRel
            | OpCode::BcsRel
            | OpCode::BeqRel
            | OpCode::BmiRel
            | OpCode::BneRel
            | OpCode::BplRel
            | OpCode::BvcRel
            | OpCode::BvsRel => Self::Rel(machine.bus.read(addr.wrapping_add(1))),

            OpCode::AdcZpg
            | OpCode::AndZpg
            | OpCode::AslZpg
            | OpCode::BitZpg
            | OpCode::CmpZpg
            | OpCode::CpxZpg
            | OpCode::CpyZpg
            | OpCode::DecZpg
            | OpCode::EorZpg
            | OpCode::IncZpg
            | OpCode::LdaZpg
            | OpCode::LdxZpg
            | OpCode::LdyZpg
            | OpCode::LsrZpg
            | OpCode::OraZpg
            | OpCode::RolZpg
            | OpCode::RorZpg
            | OpCode::SbcZpg
            | OpCode::StaZpg
            | OpCode::StxZpg
            | OpCode::StyZpg
            | OpCode::NopZpg
            | OpCode::LaxZpg
            | OpCode::SaxZpg
            | OpCode::DcpZpg => Self::Zpg(machine.bus.read(addr.wrapping_add(1))),

            OpCode::AdcZpgX
            | OpCode::AndZpgX
            | OpCode::AslZpgX
            | OpCode::CmpZpgX
            | OpCode::DecZpgX
            | OpCode::EorZpgX
            | OpCode::IncZpgX
            | OpCode::LdaZpgX
            | OpCode::LdyZpgX
            | OpCode::LsrZpgX
            | OpCode::OraZpgX
            | OpCode::RolZpgX
            | OpCode::RorZpgX
            | OpCode::SbcZpgX
            | OpCode::StaZpgX
            | OpCode::StyZpgX
            | OpCode::NopZpgX
            | OpCode::DcpZpgX => Self::ZpgX(machine.bus.read(addr.wrapping_add(1))),

            OpCode::LdxZpgY | OpCode::StxZpgY | OpCode::LaxZpgY | OpCode::SaxZpgY => {
                Self::ZpgY(machine.bus.read(addr.wrapping_add(1)))
            }
        }
    }
}
