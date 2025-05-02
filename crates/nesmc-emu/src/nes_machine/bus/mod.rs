mod apu;
mod i_ram;
mod input;
mod mapper;
mod p_ram;
mod ppu_registers;

pub use apu::Apu;
pub use i_ram::IRam;
pub use input::Input;
pub use mapper::Mapper;
pub use p_ram::PRam;
pub use ppu_registers::PpuRegisters;

pub trait Device {
    /// Reset button behavior
    fn reset(&mut self);
}

/// CPU address space trait
pub trait CpuDevice {
    fn read(&mut self, addr: u16) -> u8;

    /// Sometimes reading affects things. This is a safe debug version.
    fn read_immutable(&self, addr: u16) -> u8;

    fn write(&mut self, addr: u16, value: u8);
}

/// Ppu address space trait
pub trait PpuDevice {
    fn read_ppu(&self, addr: u16) -> u8;
    fn write_ppu(&mut self, addr: u16, value: u8);
}

#[derive(Debug, Default)]
pub struct Bus {
    pub iram: IRam,
    pub ppu_regs: PpuRegisters,
    pub apu: Apu,
    pub input: Input,

    pub pram: PRam,

    pub cart: Mapper,
}

impl Bus {
    /// Read CPU address space
    pub fn read(&mut self, addr: u16) -> u8 {
        match addr {
            0x0000..=0x1fff => self.iram.read(addr),
            0x2000..=0x3fff => self.ppu_regs.read(addr),
            0x4000..=0x4013 => self.apu.read(addr),
            0x4014 => 0,
            0x4015 => self.apu.read(addr),
            0x4016..=0x4017 => self.input.read(addr),
            0x4018..=0x401f => 0,
            0x4020..=0xffff => self.cart.read(addr),
        }
    }

    /// Sometimes reading affects things. This is a safe debug version.
    pub fn read_immutable(&self, addr: u16) -> u8 {
        match addr {
            0x0000..=0x1fff => self.iram.read_immutable(addr),
            0x2000..=0x3fff => self.ppu_regs.read_immutable(addr),
            0x4000..=0x4013 => self.apu.read_immutable(addr),
            0x4014 => 0,
            0x4015 => self.apu.read_immutable(addr),
            0x4016..=0x4017 => self.input.read_immutable(addr),
            0x4018..=0x401f => 0,
            0x4020..=0xffff => self.cart.read_immutable(addr),
        }
    }

    /// Write CPU address space
    pub fn write(&mut self, addr: u16, value: u8) {
        match addr {
            0x0000..=0x1fff => self.iram.write(addr, value),
            0x2000..=0x3fff => self.ppu_regs.write(addr, value),
            0x4000..=0x4013 => self.apu.write(addr, value),
            0x4014 => (), // TODO: OAM DMA
            0x4015 => self.apu.write(addr, value),
            0x4016 => self.input.write(addr, value),
            0x4017 => self.apu.write(addr, value),
            0x4018..=0x401f => (),
            0x4020..=0xffff => self.cart.write(addr, value),
        }
    }

    /// Read PPU address space
    pub fn read_ppu(&self, addr: u16) -> u8 {
        match addr {
            0x0000..=0x3eff => self.cart.read_ppu(addr),
            0x3f00..=0x3fff => self.pram.read_ppu(addr),
            0x4000..=0xffff => 0, // Unused range, unusable by cart?
        }
    }

    /// Write PPU address space
    pub fn write_ppu(&mut self, addr: u16, value: u8) {
        match addr {
            0x0000..=0x3eff => self.cart.write_ppu(addr, value),
            0x3f00..=0x3fff => self.pram.write_ppu(addr, value),
            0x4000..=0xffff => (), // Unused range, unusable by cart?
        }
    }

    pub fn reset(&mut self) {
        self.ppu_regs.reset();
    }
}
