use super::Bus;
use std::ops::Range;

impl Bus {
    pub fn debug_memlen(&mut self) -> usize {
        self.cpu_ram.len()
    }
    pub fn debug_memclear(&mut self) {
        let len = self.cpu_ram.len();
        //self.cpu_ram.clear();
    }
    pub fn debug_memread(&mut self, addr: u16) -> u8 {
        self.read(addr)
    }
    pub fn debug_memwrite(&mut self, addr: usize, value: u8) {
        self.cpu_ram[addr] = value;
    }
    pub fn debug_memread_range(&mut self, range: Range<usize>) -> Vec<u8> {
        let mut retvec: Vec<u8> = Vec::new();
        for addr in range {
            retvec.push(self.cpu_ram[addr])
        }
        retvec
    }
}
