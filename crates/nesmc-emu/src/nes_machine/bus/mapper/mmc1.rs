use super::MapperIo;

const KB: usize = 0x400;

#[derive(Debug)]
struct LoadReg {
    write_counter: u8,
    shift_register: u8,
}

impl Default for LoadReg {
    fn default() -> Self {
        Self {
            write_counter: 0,
            shift_register: 0,
        }
    }
}

impl LoadReg {
    const RESET_BIT: u8 = 0x80;

    pub fn write(&mut self, addr: u16, value: u8) {
        // Bit 0: Data bit
        // Bit 7: Reset bit
        // Everything between: ignore

        if value & Self::RESET_BIT > 0 {
            self.write_counter = 0;
            self.shift_register = 0;
        } else {
            self.write_counter += 1;
            self.shift_register >>= 1;
            self.shift_register |= (value & 1) << 4;

            if self.write_counter == 5 {
                match addr {
                    0x0000..=0x7fff => unreachable!(),
                    0x8000..=0x9fff => todo!(),
                    0xa000..=0xbfff => todo!(),
                    0xc000..=0xdfff => todo!(),
                    0xe000..=0xffff => todo!(),
                }
                self.write_counter = 0;
                self.shift_register = 0;
            }
        }
    }
}

#[derive(Debug)]
pub struct MMC1 {
    /// Optional 8 KB program ram
    prg_ram: Option<[u8; 8 * KB]>,
    ///
    prg_banks: Vec<[u8; 16 * KB]>,
    chr_banks: Vec<[u8; 16 * KB]>,
}

impl MMC1 {
    /// CPU RAM $6000-$7fff
    fn read_prg_ram(&self, addr: u16) -> u8 {
        let Some(ram) = self.prg_ram else {
            return 0;
        };
        ram[addr as usize]
    }

    /// CPU ROM $8000-$bfff
    fn read_prg_rom_first(&self, addr: u16) -> u8 {
        let bank = &self.prg_banks[0];
        bank[addr as usize]
    }

    /// CPU ROM $c000-$ffff
    fn read_prg_rom_second(&self, addr: u16) -> u8 {
        let bank = &self.prg_banks[0];
        bank[addr as usize]
    }
}

impl MapperIo for MMC1 {
    fn read(&self, addr: u16) -> u8 {
        match addr {
            0x0000..=0x5fff => 0,
            0x6000..=0x7fff => self.read_prg_ram(addr),
            0x8000..=0xbfff => self.read_prg_rom_first(addr),
            0xc000..=0xffff => self.read_prg_rom_second(addr),
        }
    }

    fn write(&mut self, addr: u16, _value: u8) {
        match addr {
            0x0000..=0x7fff => (),
            0x8000..=0xffff => todo!(),
        }
    }
}
