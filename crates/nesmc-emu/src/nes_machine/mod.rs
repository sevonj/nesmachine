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
}
