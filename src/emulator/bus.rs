use super::Bus;

impl Bus {
    /* --- Public --- */

    pub(crate) fn read(&mut self, addr: u16) -> u8 {
        match addr {
            0x0000..=0x1FFF => self.cpu_ram[addr as usize & 0x7ff],
            0x2000..=0x3FFF => self.read_ppu(addr & 0x7 + 0x2000),
            0x4000..=0x401f => self.read_register(addr),
            0x4020..=0xffff => self.read_cart(addr),
        }
    }
    pub(crate) fn write(&mut self, addr: u16, value: u8) {
        match addr {
            0x0000..=0x1FFF => self.cpu_ram[(addr & 0x7FF) as usize] = value,
            0x2000..=0x3FFF => self.write_ppu(addr & 0x7 + 0x2000, value),
            0x4000..=0x401f => self.write_register(addr, value),
            0x4020..=0xFFFF => self.write_cart(addr, value),
        }
    }

    pub(crate) fn read_u16(&mut self, addr: u16) -> u16 {
        let lo_byte = self.read(addr) as u16;
        let hi_byte = self.read(addr + 1) as u16;
        (hi_byte << 8) | lo_byte
    }
    pub(crate) fn read_u16_wrapped(&mut self, addr: u16) -> u16 {
        let lo_byte = self.read(addr) as u16;
        let hi_byte_addr = match addr & 0xFF == 0xFF {
            true => addr & 0xFF00,
            false => addr + 1,
        };
        let hi_byte = self.read(hi_byte_addr) as u16;
        (hi_byte << 8) | lo_byte
    }

    /* --- Private --- */

    fn read_prg_rom(&mut self, addr: u16) -> u8 {
        match self.mapper {
            0 => {
                let addr = self.mapper0_addr(addr);
                if addr >= self.prg_rom.len() {
                    println!("PRG_ROM read out of bounds: {:04x}", addr);
                    return 0;
                }
                self.prg_rom[addr]
            }
            _ => panic!("unknown mapper"),
        }
    }
    fn write_prg_rom(&mut self, addr: u16, value: u8) {
        match self.mapper {
            0 => {
                let addr = self.mapper0_addr(addr);
                self.prg_rom[addr] = value;
            }
            _ => panic!("unknown mapper"),
        }
    }

    fn mapper0_addr(&mut self, addr: u16) -> usize {
        let addr = addr as usize - 0x8000;
        match self.prg_rom.len() {
            0x4000 => addr & 0x3FFF,
            0x8000 => addr,
            _ => panic!("Mapper 0: PRG_ROM size neither 0x4000 or 0x8000."),
        }
    }
    fn read_ppu(&mut self, addr: u16) -> u8 {
        match addr {
            0x2000 => 0,
            0x2001 => 0,
            0x2002 => self.ppu.read_ppu_status(),
            0x2003 => 0,
            0x2004 => self.ppu.read_oam_data(),
            0x2005 => 0,
            0x2006 => 0,
            0x2007 => self.ppu.read_ppu_data(),
            0x4014 => 0,
            _ => panic!("PPU Read: invalid addr {}", addr),
        }
    }
    fn write_ppu(&mut self, addr: u16, value: u8) {
        match addr {
            0x2000 => self.ppu.write_ppu_ctrl(value),
            0x2001 => self.ppu.write_ppu_mask(value),
            0x2002 => (),
            0x2003 => self.ppu.write_oam_addr(value),
            0x2004 => self.ppu.write_oam_data(value),
            0x2005 => self.ppu.write_ppu_scroll(value),
            0x2006 => self.ppu.write_ppu_addr(value),
            0x2007 => self.ppu.write_ppu_data(value),
            _ => panic!("PPU Write: invalid addr {}", addr),
        }
    }
    fn read_cart(&mut self, addr: u16) -> u8 {
        match addr {
            0x4020..=0x5FFF => 0,
            0x6000..=0x7FFF => self.cart_ram[(addr - 0x6000) as usize],
            0x8000..=0xFFFF => self.read_prg_rom(addr),
            _ => panic!("Bus: Read Cart: invalid addr {}", addr),
        }
    }
    fn write_cart(&mut self, addr: u16, value: u8) {
        match addr {
            //0x4020..=0x5FFF => 0,
            0x6000..=0x7FFF => self.cart_ram[(addr - 0x6000) as usize] = value,
            0x8000..=0xFFFF => self.write_prg_rom(addr, value),
            _ => panic!("Bus: Write Cart: invalid addr {}", addr),
        }
    }

    fn read_register(&mut self, addr: u16) -> u8 {
        match addr {
            0x4016 => self.controller1.read(),
            0x4017 => self.controller2.read(),
            0x4000..=0x401f => self.apu.read(addr),
            _ => panic!("Bus: Read Register: invalid addr {}", addr),
        }
    }
    fn write_register(&mut self, addr: u16, value: u8) {
        match addr {
            0x4014 => self.ppu.write_oam_dma(value),
            0x4016 => {
                self.controller1.write(value & 0x01);
                self.controller2.write(value & 0x01);
            }
            0x4000..=0x401f => self.apu.write(addr, value),
            _ => panic!("Bus: Write Register: invalid addr {}", addr),
        }
    }

    /*
    pub(crate) fn read_old(&mut self, addr: u16) -> u8 {
        match addr {
            0x0000..=0x1FFF => self.read_ram(addr),
            0x2000..=0x3FFF => self.read_ppu(addr - 0x2000),
            0x4000..=0x4013 | 0x4015 => self.apu.read(addr),
            0x4016 => self.controller1.read(),
            0x4017 => self.controller2.read(),
            // 0x4018..0x401f is test mode
            // 0x4020 and onwards is cartridge space
            0x6000..=0x7FFF => self.cart_ram[(addr - 0x6000) as usize],
            0x8000..=0xFFFF => self.read_prg_rom(addr),
            _ => panic!("Invalid read address: 0x{:04X}", addr),
        }
    }

    pub(crate) fn write_old(&mut self, address: u16, value: u8) {
        match address {
            0x0000..=0x1FFF => self.cpu_ram[(address & 0x7FF) as usize] = value,
            0x2000..=0x3FFF => self.ppu.write(address & 0x2007, value),
            0x4000..=0x4013 | 0x4015 | 0x4017 => self.apu.write(address, value),
            0x4014 => self.ppu.write(address & 0x2007, value),
            0x4016 => {
                self.controller1.write(value & 0x01);
                self.controller2.write(value & 0x01);
            }
            0x6000..=0x7FFF => self.cart_ram[(address - 0x6000) as usize] = value,
            0x8000..=0xFFFF => self.prg_rom[(address - 0x8000) as usize] = value,
            _ => panic!("Invalid write address: 0x{:04X}", address),
        }
    }*/
}
