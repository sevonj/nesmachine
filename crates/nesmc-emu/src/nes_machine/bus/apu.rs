use super::Device;

#[derive(Debug, Default)]
pub struct APU;

impl Device for APU {
    fn read(&self, _addr: usize) -> u8 {
        0
    }

    fn write(&mut self, _addr: usize, _value: u8) {}
}
