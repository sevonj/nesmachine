use super::Device;

#[derive(Debug, Default)]
pub struct PPU;

impl Device for PPU {
    fn read(&self, _addr: usize) -> u8 {
        //let addr = addr % 0x8;
        0
    }

    fn write(&mut self, _addr: usize, _value: u8) {
        //let addr = addr % 0x8;
    }
}
