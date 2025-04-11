mod apu;
mod i_ram;
mod input;
mod mapper;
mod ppu;

pub use apu::APU;
pub use i_ram::IRam;
pub use input::Input;
pub use mapper::Mapper;
pub use ppu::PPU;

pub trait Device {
    fn read(&self, addr: usize) -> u8;
    fn write(&mut self, addr: usize, value: u8);

    // /// Reset button behavior
    // fn reset(&mut self);
}

#[derive(Debug, Default)]
pub struct Bus {
    pub iram: IRam,
    pub ppu: PPU,
    pub apu: APU,
    pub input: Input,
    pub cart: Mapper,
}

impl Bus {
    pub fn read(&self, addr: u16) -> u8 {
        match addr {
            0x0000..=0x1fff => self.iram.read(addr as usize),
            0x2000..=0x3fff => self.ppu.read(addr as usize),
            0x4000..=0x4013 => self.apu.read(addr as usize),
            0x4014 => 0,
            0x4015 => self.apu.read(addr as usize),
            0x4016..=0x4017 => self.input.read(addr as usize),
            0x4018..=0x401f => 0,
            0x4020..=0xffff => self.cart.read(addr as usize),
        }
    }

    pub fn write(&mut self, addr: u16, value: u8) {
        match addr {
            0x0000..=0x1fff => self.iram.write(addr as usize, value),
            0x2000..=0x3fff => self.ppu.write(addr as usize, value),
            0x4000..=0x4013 => self.apu.write(addr as usize, value),
            0x4014 => (), // TODO: OAM DMA
            0x4015 => self.apu.write(addr as usize, value),
            0x4016 => self.input.write(addr as usize, value),
            0x4017 => self.apu.write(addr as usize, value),
            0x4018..=0x401f => (),
            0x4020..=0xffff => self.cart.write(addr as usize, value),
        }
    }
}
