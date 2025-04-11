use super::MapperIo;

#[derive(Debug)]
pub struct MMC1 {}

impl MapperIo for MMC1 {
    fn read(&self, _addr: usize) -> u8 {
        todo!()
    }

    fn write(&mut self, _addr: usize, _value: u8) {
        todo!()
    }
}
