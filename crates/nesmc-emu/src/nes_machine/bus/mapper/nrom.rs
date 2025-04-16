use super::MapperIo;

const KIB: usize = 1024;

#[derive(Debug)]
pub struct Nrom {
    /// Optional 8 KB program ram
    prg_ram: Vec<u8>,
    prg_rom: Vec<u8>,
    _chr_rom: Vec<u8>,
}

impl Nrom {
    pub fn new(prg_rom: Vec<u8>, chr_rom: Vec<u8>) -> Self {
        Self {
            prg_ram: vec![0; 8 * KIB],
            prg_rom,
            _chr_rom: chr_rom,
        }
    }

    /// CPU RAM $6000-$7fff
    fn read_prg_ram(&self, addr: u16) -> u8 {
        let local = (addr as usize) % self.prg_ram.len();
        self.prg_ram[local]
    }

    /// CPU RAM $6000-$7fff
    fn write_prg_ram(&mut self, addr: u16, value: u8) {
        let local = (addr as usize) % self.prg_ram.len();
        self.prg_ram[local] = value
    }

    /// CPU ROM $8000-$ffff
    fn read_prg_rom(&self, addr: u16) -> u8 {
        let local = (addr as usize) % self.prg_rom.len();
        self.prg_rom[local]
    }
}

impl MapperIo for Nrom {
    fn read(&self, addr: u16) -> u8 {
        match addr {
            0x0000..=0x7fff => self.read_prg_ram(addr),
            0x8000..=0xffff => self.read_prg_rom(addr),
        }
    }

    fn write(&mut self, addr: u16, value: u8) {
        if let 0x6000..=0x7fff = addr {
            self.write_prg_ram(addr, value)
        }
    }
}
