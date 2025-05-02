#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PpuAddressKind {
    Pattern0,
    Pattern1,
    Nametable0,
    Nametable1,
    Nametable2,
    Nametable3,
    Unused,
    PaletteRam,
    PaletteRamMirror,
    Unreachable,
}

impl From<u16> for PpuAddressKind {
    fn from(value: u16) -> Self {
        match value {
            0x0000..=0x0fff => Self::Pattern0,
            0x1000..=0x1fff => Self::Pattern1,
            0x2000..=0x23ff => Self::Nametable0,
            0x2400..=0x27ff => Self::Nametable1,
            0x2800..=0x2bff => Self::Nametable2,
            0x2c00..=0x2fff => Self::Nametable3,
            0x3000..=0x3eff => Self::Unused,
            0x3F00..=0x3f1f => Self::PaletteRam,
            0x3F20..=0x3fff => Self::PaletteRamMirror,
            0x4000..=0xffff => Self::Unreachable,
        }
    }
}

impl std::fmt::Display for PpuAddressKind {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Pattern0 => write!(f, "Pattern table 0"),
            Self::Pattern1 => write!(f, "Pattern table 1"),
            Self::Nametable0 => write!(f, "Nametable 0"),
            Self::Nametable1 => write!(f, "Nametable 1"),
            Self::Nametable2 => write!(f, "Nametable 2"),
            Self::Nametable3 => write!(f, "Nametable 3"),
            Self::Unused => write!(f, "Unused (cart space)"),
            Self::PaletteRam => write!(f, "Palette RAM"),
            Self::PaletteRamMirror => write!(f, "Palette RAM (Mirror)"),
            Self::Unreachable => write!(f, "Unreachable"),
        }
    }
}

impl PpuAddressKind {
    pub fn short(&self) -> &str {
        match self {
            Self::Pattern0 => "PT0",
            Self::Pattern1 => "PT1",
            Self::Nametable0 => "NT0",
            Self::Nametable1 => "NT1",
            Self::Nametable2 => "NT2",
            Self::Nametable3 => "NT3",
            Self::Unused => "N/A",
            Self::PaletteRam => "Palet",
            Self::PaletteRamMirror => "Palet",
            Self::Unreachable => "N/A",
        }
    }

    pub fn is_mirror(&self) -> bool {
        matches!(self, Self::PaletteRamMirror)
    }
}
