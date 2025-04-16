pub mod bus;
mod cpu;
mod error;

use std::path::Path;

use bus::{Bus, Mapper};
use cpu::Cpu;
pub use error::NesMachineError;

#[derive(Debug, Default)]
pub struct NesMachine {
    pub bus: Bus,
    pub cpu: Cpu,
}

impl NesMachine {
    pub fn open<P: AsRef<Path>>(&mut self, path: P) -> Result<(), NesMachineError> {
        self.bus.cart = Mapper::None;
        self.bus.cart = Mapper::open(path)?;
        Ok(())
    }

    /// Reset button behavior
    pub fn reset(&mut self) {
        self.bus.reset();
        self.cpu.reset();
    }

    /// Step one CPU instruction
    pub fn step(&mut self) {
        self.cpu.step(&mut self.bus);
    }
}
