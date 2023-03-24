/*
 * Functions that send data from Emu to GUI thread
 *
 */

use super::Emu;
use std::ops::Range;
pub enum CtrlMSG {
    PlaybackStart,
    PlaybackStop,
    PlaybackPlayPause(bool),
    PlaybackTick,
    //DevKbdIn(i32),
    //DevGamepadState(i32),
    LoadProg(String),
    ClearMem,
    SetRate(f32),
    SetTurbo(bool),
    //SetMemSize(usize),
    GetState,
    GetRegs,
    GetMem(bool, Range<usize>),
    GetDisp,
}
pub enum ReplyMSG {
    State(EmuState),
    Regs(DebugRegs),
    Mem(Vec<u8>),
    MemSize(usize),
    Display(Vec<u8>),
    In,
    Out(i32),
}
pub struct EmuState {
    pub playing: bool,
    pub running: bool,
    pub halted: bool,
    pub speed_percent: f32,
}
pub struct DebugRegs {
    pub pc: u16,    // program counter
    pub a: u8,      // accumulator
    pub x: u8,      // x register
    pub y: u8,      // y register
    pub sp: u8,     // stack pointer
    pub status: u8, // status register
}

pub enum AddressSpace {
    CPU,
    PPU,
}
/*
impl Default for DebugRegs {
    fn default() -> Self {
        DebugRegs {
            pc: 0,
            a: 0,
            x: 0,
            y: 0,
            sp: 0,
            status: 0,
        }
    }
}*/

impl Emu {
    pub fn debug_sendstate(&mut self) {
        self.tx.send(ReplyMSG::State(EmuState {
            playing: self.playing,
            running: self.running,
            halted: false, //self.cpu.debug_get_halt(),
            speed_percent: self.perfmon.get_percent(),
        }));
    }

    pub fn debug_sendmem(&mut self, ppu_space: bool, range: Range<usize>) {
        let mut retvec: Vec<u8> = Vec::with_capacity(range.len());
        match ppu_space {
            false => {
                for i in range.clone() {
                    if i >= 0xFFFF {
                        break;
                    }
                    retvec.push(self.bus.debug_memread(i as u16));
                }
            }
            true => {
                for i in range.clone() {
                    if i >= 0xFFFF {
                        break;
                    }
                    retvec.push(self.bus.ppu.debug_read(i as u16));
                }
            }
        }

        self.tx.send(ReplyMSG::Mem(retvec));
        self.tx.send(ReplyMSG::MemSize(u16::MAX as usize + 1));
    }

    pub fn debug_sendregs(&mut self) {
        self.tx.send(ReplyMSG::Regs(DebugRegs {
            pc: self.cpu.pc,
            a: self.cpu.a,
            x: self.cpu.x,
            y: self.cpu.y,
            sp: self.cpu.sp,
            status: self.cpu.status,
        }));
    }

    pub fn debug_senddisp(&mut self) {
        // //let range = 8192..8192 + 120 * 160;
        // let retvec = Vec::<i32>::new(); // self.cpu.debug_memread_range(range).to_vec();
        // self.tx.send(ReplyMSG::Display(retvec));
    }
}
