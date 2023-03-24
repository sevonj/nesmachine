use std::sync::mpsc::{Receiver, Sender};
use std::thread;
use std::time::{Duration, Instant};
mod bus;
mod bus_debug;
pub mod emu_debug;
mod perfmon;

use self::cpu::Cpu;
use self::emu_debug::{AddressSpace, CtrlMSG, ReplyMSG};
use self::perfmon::PerfMonitor;
mod loader;

mod apu;
mod ppu;
use apu::APU;
use ppu::PPU;
mod controller;
use controller::Controller;

pub fn run(tx: Sender<ReplyMSG>, rx: Receiver<CtrlMSG>) {
    let mut emu = Emu::default(tx, rx);
    loop {
        emu.update();
    }
}

/*
Emulator
├── cpu
└── bus
    ├── cpu_ram
    ├── prg_rom
    ├── cart_ram
    ├── ppu
    │   ├── chr_rom
    │   └── video_ram
    ├── apu
    ├── controller1
    └── controller2
*/

mod cpu;

pub(crate) struct Bus {
    cpu_ram: [u8; 0x0800],
    prg_rom: Vec<u8>,
    cart_ram: [u8; 0x2000],
    pub(crate) ppu: PPU,
    apu: APU,
    controller1: Controller,
    controller2: Controller,
    mapper: u32,
}

impl Bus {
    fn new() -> Self {
        Bus {
            cpu_ram: [0; 0x0800],
            prg_rom: vec![0; 0x4000],
            cart_ram: [0; 0x2000],
            ppu: PPU::new(),
            apu: APU::new(),
            controller1: Controller::new(),
            controller2: Controller::new(),
            mapper: 0,
        }
    }
}

pub struct Emu {
    cpu: Cpu,
    bus: Bus,

    tx: Sender<ReplyMSG>,
    rx: Receiver<CtrlMSG>,
    loaded_prog: String,
    running: bool,
    playing: bool,
    // Time
    turbo: bool,
    t_last_update: Option<Instant>,
    perfmon: PerfMonitor,
    // CPU
    cpu_rate: f32,
    cpu_timer: Duration,
    cpu_last_tick: Option<Instant>,
    // PPU
    ppu_rate: f32,
    ppu_timer: Duration,
    // Test Log
    test_log: Option<String>,
    test_log_line: usize,
    last_ins: u8,
    stop_on_mismatch: bool,
    exec_breakpoints: Vec<u16>,
}
impl Emu {
    pub fn default(tx: Sender<ReplyMSG>, rx: Receiver<CtrlMSG>) -> Self {
        Emu {
            cpu: Cpu::new(),
            bus: Bus::new(),

            tx,
            rx,
            loaded_prog: String::new(),
            running: false,
            playing: false,
            turbo: false,
            t_last_update: None,
            perfmon: PerfMonitor::default(),
            // CPU
            cpu_rate: 10.,
            cpu_timer: Duration::ZERO,
            cpu_last_tick: None,
            // PPU
            ppu_rate: 60.,
            ppu_timer: Duration::ZERO,
            // Test Log
            test_log: None,
            test_log_line: 1,
            last_ins: 0,
            stop_on_mismatch: true,
            exec_breakpoints: Vec::new(),
        }
    }

    pub fn update(&mut self) {
        self.timekeeper();
        self.check_mail();
        if self.playing {
            let cpu_tick_duration = Duration::from_secs_f32(1. / self.cpu_rate);
            let ppu_tick_duration = Duration::from_secs_f32(1. / self.ppu_rate);
            /*if self.turbo {
                // Turbomode: No limits!
                self.tick_timer = Duration::ZERO;
                self.tick();

            } else {*/
            // Normomode: Wait for tick timer
            if self.ppu_timer >= ppu_tick_duration {
                self.ppu_timer -= ppu_tick_duration;
                self.bus.ppu.render();
                self.tx
                    .send(ReplyMSG::Display(self.bus.ppu.framebuffer.clone()));
            }
            if self.cpu_timer >= cpu_tick_duration {
                self.cpu_timer -= cpu_tick_duration;
                self.tick();
            } else {
                // If no tick, sleep
                thread::sleep(Duration::from_secs_f32(0.5 / self.cpu_rate))
            }
        } else {
            // Sleep longer when not playing
            thread::sleep(Duration::from_secs_f32(1. / 60.));
        }
    }

    fn timekeeper(&mut self) {
        let now = Instant::now();
        let delta;
        match self.t_last_update {
            Some(last) => delta = now - last,
            None => delta = Duration::ZERO,
        }
        self.t_last_update = Some(now);
        if self.playing {
            self.cpu_timer += delta;
            self.ppu_timer += delta;
        }
        self.perfmon.set_rate(self.cpu_rate);
        //if let Some(d) = self.interrupt_timer {
        //    self.interrupt_timer = Some(d - delta);
        //    if d == Duration::ZERO {
        //        self.interrupt_timer = None;
        //        self.cpu.cu_sr |= SR_I;
        //    }
        //}
    }

    fn check_mail(&mut self) {
        // Loop until there are no messages, because messages may
        // come faster than update.
        loop {
            if let Ok(msg) = self.rx.try_recv() {
                match msg {
                    // Playback control
                    CtrlMSG::PlaybackStart => self.start(),
                    CtrlMSG::PlaybackStop => self.stop(),
                    CtrlMSG::PlaybackPlayPause(p) => self.playpause(p),
                    CtrlMSG::PlaybackTick => self.tick(),
                    // Dev
                    //CtrlMSG::DevKbdIn(input) => self.cpu.input_handler(input),
                    //CtrlMSG::DevGamepadState(_input) => todo!(),
                    // Loader
                    CtrlMSG::LoadProg(fname) => self.loadprog(fname),
                    CtrlMSG::ClearMem => self.clearmem(),
                    // Settings
                    CtrlMSG::SetRate(rate) => self.cpu_rate = rate,
                    CtrlMSG::SetTurbo(t) => self.turbo = t,
                    //CtrlMSG::SetMemSize(size) => self.cpu.debug_memresize(size),
                    // Debug
                    CtrlMSG::GetState => self.debug_sendstate(),
                    CtrlMSG::GetMem(ppu_space, range) => self.debug_sendmem(ppu_space, range),
                    CtrlMSG::GetRegs => self.debug_sendregs(),
                    CtrlMSG::GetDisp => self.debug_senddisp(),
                }
            } else {
                break;
            }
        }
    }

    fn start(&mut self) {
        self.reload();
        self.add_breakpoints();
        //self.cpu.debug_clear_cu();
        self.running = true;
        //self.cpu.debug_set_halt(false);
        //self.cpu.debug_clear_fire();
        self.t_last_update = None;
    }

    fn stop(&mut self) {
        self.t_last_update = None;
        self.running = false;
        self.playing = false;
        self.debug_sendregs();
        self.debug_sendstate();
    }

    fn playpause(&mut self, p: bool) {
        self.t_last_update = None;
        self.playing = p;
    }

    fn loadprog(&mut self, filename: String) {
        self.stop();
        self.loaded_prog = filename.clone();
        self.bus.load_rom(&self.loaded_prog);
        self.cpu.reset(&mut self.bus);
        self.loadtestlog(filename)
    }
    fn loadtestlog(&mut self, filename: String) {
        return;
        let mut log_path = std::path::PathBuf::from(filename);
        if log_path.extension() == None {
            log_path.push("");
            log_path.set_extension("log");
        } else {
            log_path.set_extension("log");
        }
        match std::fs::read_to_string(log_path) {
            Ok(content) => self.test_log = Some(content),
            Err(_) => self.test_log = None,
        }
    }

    fn check_breakpoint(&mut self) {
        if self.exec_breakpoints.contains(&self.cpu.pc) {
            self.playing = false;
            println!("Breakpoint Exec: {:x}", self.cpu.pc);
        }
    }

    fn add_breakpoints(&mut self) {
        //self.exec_breakpoints.push(0xc94d);
        //self.exec_breakpoints.push(0xCF4E);
    }
    fn test_against_log(&mut self) {
        let line;
        match &self.test_log {
            Some(log) => match log.lines().nth(self.test_log_line) {
                Some(ln) => line = ln,
                None => return,
            },
            None => return,
        }
        self.test_log_line += 1;

        let pc_str = &line[0..4];
        let pc = u16::from_str_radix(pc_str, 16).expect("Invalid PC value in test log line");

        let a_idx = line
            .find("A:")
            .expect("Could not find 'A:' in test log line");
        let a_str = &line[a_idx + 2..a_idx + 4];
        let a = u8::from_str_radix(a_str, 16).expect("Invalid A value in test log line");

        let x_idx = line
            .find("X:")
            .expect("Could not find 'X:' in test log line");
        let x_str = &line[x_idx + 2..x_idx + 4];
        let x = u8::from_str_radix(x_str, 16).expect("Invalid X value in test log line");

        let y_idx = line
            .find("Y:")
            .expect("Could not find 'Y:' in test log line");
        let y_str = &line[y_idx + 2..y_idx + 4];
        let y = u8::from_str_radix(y_str, 16).expect("Invalid Y value in test log line");

        let sp_idx = line
            .find("SP:")
            .expect("Could not find 'SP:' in test log line");
        let sp_str = &line[sp_idx + 3..sp_idx + 5];
        let sp = u8::from_str_radix(sp_str, 16).expect("Invalid SP value in test log line");

        if self.cpu.pc != pc {
            if self.stop_on_mismatch {
                self.stop();
            }
            println!("ERROR! PC cpu: {:x}, log: {:x}", self.cpu.pc, pc);
            println!(
                "last instruction: {:x}. Line: {}",
                self.last_ins, self.test_log_line
            );
        }
        if self.cpu.a != a {
            if self.stop_on_mismatch {
                self.stop();
            }
            println!("ERROR! A cpu: {:x}, log: {:x}", self.cpu.a, a);
            println!(
                "last instruction: {:x}. Line: {}",
                self.last_ins, self.test_log_line
            );
        }
        if self.cpu.x != x {
            if self.stop_on_mismatch {
                self.stop();
            }
            println!("ERROR! X cpu: {:x}, log: {:x}", self.cpu.x, x);
            println!(
                "last instruction: {:x}. Line: {}",
                self.last_ins, self.test_log_line
            );
        }
        if self.cpu.y != y {
            if self.stop_on_mismatch {
                self.stop();
            }
            println!("ERROR! Y cpu: {:x}, log: {:x}", self.cpu.y, y);
            println!(
                "last instruction: {:x}. Line: {}",
                self.last_ins, self.test_log_line
            );
        }
        if self.cpu.sp != sp {
            if self.stop_on_mismatch {
                self.stop();
            }
            println!("ERROR! SP cpu: {:x}, log: {:x}", self.cpu.sp, sp);
            println!(
                "last instruction: {:x}. Line: {}",
                self.last_ins, self.test_log_line
            );
        }
    }

    fn reload(&mut self) {

        //loader::load_program(&mut self.cpu, &self.loaded_prog);
    }

    fn clearmem(&mut self) {
        self.stop();
        //self.cpu.debug_memclear();
    }

    fn tick(&mut self) {
        //if self.cpu.input_wait != None || self.cpu.debug_get_halt() ||
        if !self.running {
            return;
        }
        self.last_ins = self.bus.read(self.cpu.pc);
        self.perfmon.tick();
        self.cpu_last_tick = Some(Instant::now());
        self.cpu.tick(&mut self.bus);
        self.test_against_log();
        self.check_breakpoint();

        //if let Some(dev) = self.cpu.input_wait {
        //    self.dev_read(dev)
        //}
        //if let Some((dev, val)) = self.cpu.output {
        //    self.dev_write(dev, val);
        //    self.cpu.output = None;
        //}
        //if self.cpu.debug_is_on_fire() {
        //    self.playing = false;
        //    self.running = false;
        //}
    }

    /*fn dev_read(&mut self, dev: i32) {
        match dev {
            DEV_CRT => {
                println!("You can't read from crt!");
                self.cpu.input_handler(0);
            }
            DEV_KBD => {
                self.tx.send(ReplyMSG::In);
            }
            DEV_RTC => {
                let time =
                    Local::now().timestamp() as i32 + Local::now().offset().local_minus_utc();
                self.cpu.input_handler(time);
            }
            _ => {
                println!("Attempted to read from an unknown device");
                self.cpu.input_handler(0);
            }
        }
    }
    fn dev_write(&mut self, dev: i32, val: i32) {
        match dev {
            DEV_CRT => {
                self.tx.send(ReplyMSG::Out(val));
            }
            DEV_KBD => {
                println!("You can't output to a keyboard!");
            }
            DEV_RTC => {
                println!("You can't output to RTC!");
            }
            _ => {
                println!("Attempted to write into an unknown device");
            }
        }
    }*/
}
