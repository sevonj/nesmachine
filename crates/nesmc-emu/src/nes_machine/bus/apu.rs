use super::Device;

#[derive(Debug, Default)]
pub struct APU;

impl Device for APU {
    fn read(&self, _addr: u16) -> u8 {
        0
    }

    fn write(&mut self, _addr: u16, _value: u8) {}
}
