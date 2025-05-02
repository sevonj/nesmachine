use super::CpuDevice;

#[derive(Debug, Default)]
pub struct Input;

impl CpuDevice for Input {
    fn read(&mut self, _addr: u16) -> u8 {
        0
    }

    fn read_immutable(&self, _addr: u16) -> u8 {
        0
    }

    fn write(&mut self, _addr: u16, _value: u8) {}
}
