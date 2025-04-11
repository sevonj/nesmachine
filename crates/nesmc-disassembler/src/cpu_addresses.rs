#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CpuAddressKind {
    IRam,
    IRamMirror,
    PpuReg,
    PpuRegMirror,
    ApuIo,
    Disabled,
    Cart,
}

impl From<u16> for CpuAddressKind {
    fn from(value: u16) -> Self {
        match value {
            0x0000..=0x07ff => Self::IRam,
            0x0800..=0x1fff => Self::IRamMirror,
            0x2000..=0x2007 => Self::PpuReg,
            0x2008..=0x3fff => Self::PpuRegMirror,
            0x4000..=0x4017 => Self::ApuIo,
            0x4018..=0x401f => Self::Disabled,
            0x4020..=0xffff => Self::Cart,
        }
    }
}

impl std::fmt::Display for CpuAddressKind {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::IRam => write!(f, "Internal RAM"),
            Self::IRamMirror => write!(f, "Internal RAM (mirror)"),
            Self::PpuReg => write!(f, "PPU Register"),
            Self::PpuRegMirror => write!(f, "PPU Register (Mirror)"),
            Self::ApuIo => write!(f, "APU or IO Register"),
            Self::Disabled => write!(f, "Disabled (test mode or unused)"),
            Self::Cart => write!(f, "Cartridge space"),
        }
    }
}

impl CpuAddressKind {
    pub fn short(&self) -> &str {
        match self {
            Self::IRam => "IRAM",
            Self::IRamMirror => "IRAM",
            Self::PpuReg => "PPU Reg",
            Self::PpuRegMirror => "PPU Reg",
            Self::ApuIo => "APU/IO",
            Self::Disabled => "DISABLED",
            Self::Cart => "CART",
        }
    }

    pub fn is_mirror(&self) -> bool {
        matches!(self, Self::IRamMirror | Self::PpuRegMirror)
    }
}
