//! Cartridge / "rom" module

mod mmc1;
mod nrom;

use std::{
    fs::File,
    io::{BufReader, Read, Seek},
    path::Path,
};

pub use mmc1::Mmc1;
use nrom::Nrom;

use crate::nes_machine::NesMachineError;

use super::{CpuDevice, PpuDevice};

pub trait MapperIo {
    fn read_cpu(&self, addr: u16) -> u8;
    fn write_cpu(&mut self, addr: u16, value: u8);
    fn read_ppu(&self, addr: u16) -> u8;
    fn write_ppu(&mut self, addr: u16, value: u8);
    fn arrangement(&self) -> NametableArrangement;
}

#[non_exhaustive]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum NametableArrangement {
    OneScreenLower,
    OneScreenUpper,
    /// Vertical mirroring
    HorizontalArrangement,
    /// Horizontal mirroring
    VerticalArrangement,
}

impl NametableArrangement {
    pub fn from_sr_byte(value: u8) -> Self {
        match value | 0x03 {
            0 => Self::OneScreenLower,
            1 => Self::OneScreenUpper,
            2 => Self::HorizontalArrangement,
            3 => Self::VerticalArrangement,
            _ => unreachable!(),
        }
    }

    fn is_mirror(&self, addr: u16) -> bool {
        match self {
            Self::OneScreenLower => matches!(addr, 0x2400..=0x2fff),
            Self::OneScreenUpper => matches!(addr,
                0x2000..=0x23ff |
                0x2800..=0x2fff ),
            Self::HorizontalArrangement => matches!(addr, 0x2800..=0x2fff),
            Self::VerticalArrangement => matches!(addr,
                0x2400..=0x27ff |
                0x2c00..=0x2fff ),
        }
    }

    /// Remap PPU address according to mirroring setup
    const fn map_addr(&self, addr: u16) -> u16 {
        match self {
            Self::OneScreenLower => match addr {
                // 0x2000..=0x23ff
                0x2400..=0x27ff => addr - 0x400,
                0x2800..=0x2bff => addr - 0x800,
                0x2c00..=0x2fff => addr - 0xc00,
                _ => addr,
            },
            Self::OneScreenUpper => match addr {
                0x2000..=0x23ff => addr + 0x400,
                // 0x2400..=0x27ff
                0x2800..=0x2bff => addr - 0x400,
                0x2c00..=0x2fff => addr - 0x800,
                _ => addr,
            },
            Self::HorizontalArrangement => match addr {
                // 0x2000..=0x23ff
                // 0x2400..=0x27ff
                0x2800..=0x2fff => addr - 0x800,
                _ => addr,
            },
            Self::VerticalArrangement => match addr {
                // 0x2000..=0x23ff
                0x2400..=0x27ff => addr - 0x400,
                0x2800..=0x2bff => addr - 0x400,
                0x2c00..=0x2fff => addr - 0x800,
                _ => addr,
            },
        }
    }

    pub fn bank_id(&self, addr: u16) -> Option<&'static str> {
        match self.map_addr(addr) {
            0x2000..=0x23ff => Some("A"),
            0x2400..=0x27ff => Some("B"),
            0x2800..=0x2bff => Some("C"),
            0x2c00..=0x2fff => Some("D"),
            _ => None,
        }
    }
}

#[derive(Debug)]
pub struct INesHeader {
    len_prg_rom: usize,
    len_chr_rom: usize,
    mapper_id: u8,

    v_mirroring: bool,
}

impl INesHeader {
    const MAGIC: &[u8; 4] = b"NES\x1a";

    pub fn read<R: Read + Seek>(reader: &mut BufReader<R>) -> Result<Self, NesMachineError> {
        let mut header_buf = [0_u8; 16];
        reader.read_exact(&mut header_buf)?;

        if header_buf[0..=3] != *Self::MAGIC {
            return Err(NesMachineError::FileInvalidSig);
        }

        let len_prg_rom = header_buf[4] as usize * 16384;
        let len_chr_rom = header_buf[5] as usize * 8192;
        let mapper_id = (header_buf[6] >> 4) + (header_buf[7] & 0xf0);

        let v_mirroring = header_buf[6] & 0x1 != 0;
        let battery = header_buf[6] & 0x2 != 0;
        let trainer = header_buf[6] & 0x4 != 0;
        let alt_nametable_layout = header_buf[6] & 0x8 != 0;

        let vs_unisystem = header_buf[7] & 0x1 != 0;
        let playchoice = header_buf[7] & 0x2 != 0;
        let nes2_0 = (header_buf[7] >> 2) & 0x3 == 2;

        if battery | trainer | alt_nametable_layout | vs_unisystem | playchoice | nes2_0 {
            return Err(NesMachineError::MapperUnsupportedFeatures);
        }

        Ok(Self {
            len_prg_rom,
            len_chr_rom,
            mapper_id,
            v_mirroring,
        })
    }
}

// Lint complaint: size difference between types. It's okay at least for now.
#[allow(clippy::large_enum_variant)]
#[derive(Debug, Default)]
pub enum Mapper {
    #[default]
    None,
    Nrom(Nrom),
    Mmc1(Mmc1),
}

impl CpuDevice for Mapper {
    fn read(&mut self, addr: u16) -> u8 {
        self.read_immutable(addr)
    }

    fn read_immutable(&self, addr: u16) -> u8 {
        match self {
            Mapper::None => 0,
            Mapper::Nrom(nrom) => nrom.read_cpu(addr),
            Mapper::Mmc1(mmc1) => mmc1.read_cpu(addr),
        }
    }

    fn write(&mut self, addr: u16, value: u8) {
        match self {
            Mapper::None => (),
            Mapper::Nrom(nrom) => nrom.write_cpu(addr, value),
            Mapper::Mmc1(mmc1) => mmc1.write_cpu(addr, value),
        }
    }
}

impl PpuDevice for Mapper {
    fn read_ppu(&self, addr: u16) -> u8 {
        match self {
            Mapper::None => 0,
            Mapper::Nrom(nrom) => nrom.read_ppu(addr),
            Mapper::Mmc1(mmc1) => mmc1.read_ppu(addr),
        }
    }

    fn write_ppu(&mut self, addr: u16, value: u8) {
        match self {
            Mapper::None => (),
            Mapper::Nrom(nrom) => nrom.write_ppu(addr, value),
            Mapper::Mmc1(mmc1) => mmc1.write_ppu(addr, value),
        }
    }
}

impl Mapper {
    pub fn open<P: AsRef<Path>>(path: P) -> Result<Self, NesMachineError> {
        let mut reader = BufReader::new(File::open(path)?);
        Self::from_reader(&mut reader)
    }

    pub fn from_reader<R: Read + Seek>(reader: &mut BufReader<R>) -> Result<Self, NesMachineError> {
        let header = INesHeader::read(reader)?;

        println!("{header:x?}");

        match header.mapper_id {
            0 => {
                // 16KB OR 32KB
                let mut prg_rom = vec![0_u8; header.len_prg_rom];
                match header.len_prg_rom {
                    0x4000 => reader.read_exact(&mut prg_rom)?,
                    0x8000 => reader.read_exact(&mut prg_rom)?,
                    _ => {
                        return Err(NesMachineError::MapperUnexpectedPrgRomLen(
                            header.len_chr_rom,
                        ));
                    }
                }

                // 8KB
                let mut chr_rom = vec![0_u8; 0x2000];
                match header.len_chr_rom {
                    0x2000 => reader.read_exact(&mut chr_rom)?,
                    _ => {
                        return Err(NesMachineError::MapperUnexpectedChrRomLen(
                            header.len_chr_rom,
                        ));
                    }
                }

                Ok(Self::Nrom(Nrom::new(prg_rom, chr_rom, header.v_mirroring)))
            }
            1 => {
                let prg_ram = Some(vec![0_u8; 32 * 1024]);
                let mut prg_rom = vec![0_u8; header.len_prg_rom];
                let mut chr_rom = vec![0_u8; header.len_chr_rom];

                reader.read_exact(&mut prg_rom)?;
                reader.read_exact(&mut chr_rom)?;

                Ok(Self::Mmc1(Mmc1::new(prg_ram, prg_rom, chr_rom)))
            }
            _ => Err(NesMachineError::MapperUnsupportedId(
                header.mapper_id as usize,
            )),
        }
    }

    pub fn nt_arrangement(&self) -> Option<NametableArrangement> {
        match self {
            Mapper::None => None,
            Mapper::Nrom(nrom) => Some(nrom.arrangement()),
            Mapper::Mmc1(mmc1) => Some(mmc1.arrangement()),
        }
    }

    pub fn is_ppu_addr_mirror(&self, addr: u16) -> bool {
        match self {
            Mapper::None => false,
            Mapper::Nrom(nrom) => nrom.arrangement().is_mirror(addr),
            Mapper::Mmc1(mmc1) => mmc1.arrangement().is_mirror(addr),
        }
    }
}
