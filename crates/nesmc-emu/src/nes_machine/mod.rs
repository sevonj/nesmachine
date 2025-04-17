pub mod bus;
mod cpu;
mod error;

use std::path::Path;

use bus::{Bus, Mapper};
use cpu::Cpu;
pub use error::NesMachineError;

#[derive(Debug)]
pub struct NesMachine {
    pub bus: Bus,
    pub cpu: Cpu,
    pub cycle_count: usize,
}

impl Default for NesMachine {
    fn default() -> Self {
        let bus = Bus::default();
        let cpu = Cpu::new(&bus);

        Self {
            bus,
            cpu,
            cycle_count: 7,
        }
    }
}

impl NesMachine {
    pub fn open<P: AsRef<Path>>(&mut self, path: P) -> Result<(), NesMachineError> {
        self.bus.cart = Mapper::None;
        self.bus.cart = Mapper::open(path)?;
        self.cpu = Cpu::new(&self.bus);
        Ok(())
    }

    /// Reset button behavior
    pub fn reset(&mut self) {
        self.bus.reset();
        self.cpu.reset(&self.bus);
    }

    /// Step one CPU instruction
    pub fn step(&mut self) {
        self.cycle_count += self.cpu.step(&mut self.bus);
    }
}

#[cfg(test)]
mod tests {
    use std::fs::read_to_string;

    use super::cpu::*;
    use super::*;

    fn parse_cpu(line: &str) -> Cpu {
        Cpu {
            a: u8::from_str_radix(&line[50..=51], 16).unwrap(),
            x: u8::from_str_radix(&line[55..=56], 16).unwrap(),
            y: u8::from_str_radix(&line[60..=61], 16).unwrap(),
            pc: u16::from_str_radix(&line[0..=3], 16).unwrap(),
            sp: u8::from_str_radix(&line[71..=72], 16).unwrap(),
            status: CpuStatus::from(u8::from_str_radix(&line[65..=66], 16).unwrap()),
        }
    }

    fn parse_cycles(line: &str) -> usize {
        usize::from_str_radix(&line[90..], 10).unwrap()
    }

    fn run_against_log(
        machine: &mut NesMachine,
        log: String,
        fail_on_mismatch: bool,
        cycle_accurate: bool,
    ) {
        for (i, line) in log.lines().enumerate() {
            let ln = i + 1;

            if ln == 4270 {
                println!("0x0078:{:02x}", machine.bus.read(0x78));
            }

            let ref_cpu = parse_cpu(line);
            let ref_cyc = parse_cycles(line);
            let this_cpu = &machine.cpu;
            let this_cyc = machine.cycle_count;

            if fail_on_mismatch && (ref_cpu != *this_cpu) {
                panic!("ln {ln:02} CPU STATE MISMATCH\n    ref: {ref_cpu}\n    got: {this_cpu}\n",);
            }
            if cycle_accurate && (this_cyc != ref_cyc) {
                panic!("Cycle mismach\n    ref: {ref_cyc}\n    got: {this_cyc}\n");
            }

            println!("ln {ln:02} ok - {this_cpu} CYC:{this_cyc}");
            machine.step();
        }
    }

    #[test]
    fn run_nestest_c000_log() {
        let mut machine = NesMachine::default();
        machine.open("../../tests/nestest.nes").unwrap();
        machine.cpu.pc = 0xc000;

        let log = read_to_string("../../tests/nestest_c000.log").unwrap();
        run_against_log(&mut machine, log, true, false);
    }

    #[test]
    fn run_nestest_c000_log_cycles() {
        let mut machine = NesMachine::default();
        machine.open("../../tests/nestest.nes").unwrap();
        machine.cpu.pc = 0xc000;

        let log = read_to_string("../../tests/nestest_c000.log").unwrap();
        run_against_log(&mut machine, log, true, true);
    }

    /*
    #[test]
    fn run_nestest_c000_auto_legal() {
        let mut machine = NesMachine::default();
        machine.open("../../tests/nestest.nes").unwrap();
        machine.cpu.pc = 0xc000;

        let log = read_to_string("../../tests/nestest_c000.log").unwrap();
        run_against_log(&mut machine, log, false);

        assert_eq!(machine.bus.read(0x02), 0);
    }
    // */
}
