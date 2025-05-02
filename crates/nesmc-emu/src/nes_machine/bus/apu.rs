use super::CpuDevice;

#[derive(Debug, Default)]
pub struct Apu;

impl CpuDevice for Apu {
    fn read(&mut self, _addr: u16) -> u8 {
        0
    }

    fn read_immutable(&self, _addr: u16) -> u8 {
        0
    }

    fn write(&mut self, _addr: u16, _value: u8) {}
}
