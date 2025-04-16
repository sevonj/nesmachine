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

use super::Device;

#[derive(Debug)]
pub struct INesHeader {
    len_prg_rom: usize,
    len_chr_rom: usize,
    mapper_id: u8,
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

        let mirroring = header_buf[6] & 0x1 != 0;
        let battery = header_buf[6] & 0x2 != 0;
        let trainer = header_buf[6] & 0x4 != 0;
        let alt_nametable_layout = header_buf[6] & 0x8 != 0;

        let vs_unisystem = header_buf[7] & 0x1 != 0;
        let playchoice = header_buf[7] & 0x2 != 0;
        let nes2_0 = (header_buf[7] >> 2) & 0x3 == 2;

        if mirroring | battery | trainer | alt_nametable_layout | vs_unisystem | playchoice | nes2_0
        {
            return Err(NesMachineError::UnsupportedMapper);
        }

        Ok(Self {
            len_prg_rom,
            len_chr_rom,
            mapper_id,
        })
    }
}

pub trait MapperIo {
    fn read(&self, addr: u16) -> u8;
    fn write(&mut self, addr: u16, value: u8);
}

#[derive(Debug, Default)]
pub enum Mapper {
    #[default]
    None,
    Nrom(Nrom),
    Mmc1(Mmc1),
}

impl Device for Mapper {
    fn read(&self, addr: u16) -> u8 {
        match self {
            Mapper::None => 0,
            Mapper::Nrom(nrom) => nrom.read(addr),
            Mapper::Mmc1(mmc1) => mmc1.read(addr),
        }
    }

    fn write(&mut self, addr: u16, value: u8) {
        match self {
            Mapper::None => (),
            Mapper::Nrom(nrom) => nrom.write(addr, value),
            Mapper::Mmc1(mmc1) => mmc1.write(addr, value),
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

        println!("{header:?}");

        match header.mapper_id {
            0 => {
                let mut prg_rom = vec![0_u8; header.len_prg_rom];
                let mut chr_rom = vec![0_u8; header.len_chr_rom];

                reader.read_exact(&mut prg_rom)?;
                reader.read_exact(&mut chr_rom)?;

                Ok(Self::Nrom(Nrom::new(prg_rom, chr_rom)))
            }
            1 => {
                let prg_ram = Some(vec![0_u8; 32 * 1024]);
                let mut prg_rom = vec![0_u8; header.len_prg_rom];
                let mut chr_rom = vec![0_u8; header.len_chr_rom];

                reader.read_exact(&mut prg_rom)?;
                reader.read_exact(&mut chr_rom)?;

                Ok(Self::Mmc1(Mmc1::new(prg_ram, prg_rom, chr_rom)))
            }
            _ => Err(NesMachineError::UnsupportedMapper),
        }
    }
}
