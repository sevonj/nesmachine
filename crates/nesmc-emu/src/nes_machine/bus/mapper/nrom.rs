use super::{MapperIo, NametableArrangement};

#[derive(Debug)]
pub struct Nrom {
    /// CPU 0x8000..=0xffff
    prg_rom: Vec<u8>,
    /// PPU 0x0000..=0x1fff
    chr_rom: Vec<u8>,
    /// Belongs to console, but routed by mapper.
    vram: [u8; 0x800],
    arrangement: NametableArrangement,
}

impl Nrom {
    pub fn new(prg_rom: Vec<u8>, chr_rom: Vec<u8>, v_mirroring: bool) -> Self {
        Self {
            prg_rom,
            chr_rom,
            vram: [0; 0x800],
            arrangement: if v_mirroring {
                NametableArrangement::HorizontalArrangement
            } else {
                NametableArrangement::VerticalArrangement
            },
        }
    }

    fn map_prg_rom_mirror(&self, addr: u16) -> usize {
        (addr as usize - 0x8000) % self.prg_rom.len()
    }
}

impl MapperIo for Nrom {
    fn read_cpu(&self, addr: u16) -> u8 {
        match addr {
            0x0000..=0x7fff => 0,
            0x8000..=0xffff => self.prg_rom[self.map_prg_rom_mirror(addr)],
        }
    }

    fn write_cpu(&mut self, _addr: u16, _value: u8) {}

    fn read_ppu(&self, addr: u16) -> u8 {
        let addr = self.arrangement.map_addr(addr);
        match addr {
            0x0000..=0x1fff => self.chr_rom[addr as usize],
            _ => 0,
        }
    }

    fn write_ppu(&mut self, addr: u16, value: u8) {
        let addr = self.arrangement.map_addr(addr);
        if let 0x2000..=0x27ff = addr {
            self.vram[addr as usize] = value
        }
    }

    fn arrangement(&self) -> NametableArrangement {
        self.arrangement
    }
}
