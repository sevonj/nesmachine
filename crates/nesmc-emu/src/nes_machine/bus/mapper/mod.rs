//! Cartridge / "rom" module

mod mmc1;

use std::{
    fs::File,
    io::{BufReader, Read, Seek},
    path::Path,
};

pub use mmc1::MMC1;

use crate::nes_machine::NesMachineError;

use super::Device;

#[derive(Debug)]
pub struct INesHeader {
    _len_prg_rom: u8,
    _len_chr_rom: u8,
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

        let _len_prg_rom = header_buf[4];
        let _len_chr_rom = header_buf[5];
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
            _len_prg_rom,
            _len_chr_rom,
            mapper_id,
        })
    }
}

pub trait MapperIo {
    fn read(&self, addr: usize) -> u8;
    fn write(&mut self, addr: usize, value: u8);
}

#[derive(Debug, Default)]
pub enum Mapper {
    #[default]
    None,
    MMC1(MMC1),
}

impl Device for Mapper {
    fn read(&self, addr: usize) -> u8 {
        match self {
            Mapper::None => 0,
            Mapper::MMC1(mmc1) => mmc1.read(addr),
        }
    }

    fn write(&mut self, addr: usize, value: u8) {
        match self {
            Mapper::None => (),
            Mapper::MMC1(mmc1) => mmc1.write(addr, value),
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
            1 => todo!(),
            _ => Err(NesMachineError::UnsupportedMapper),
        }
    }
}
