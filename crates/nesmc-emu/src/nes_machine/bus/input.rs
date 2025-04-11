use super::Device;

#[derive(Debug, Default)]
pub struct Input;

impl Device for Input {
    fn read(&self, _addr: usize) -> u8 {
        0
    }

    fn write(&mut self, _addr: usize, _value: u8) {}
}
