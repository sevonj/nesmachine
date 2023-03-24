use crate::{emulator::emu_debug::CtrlMSG, NesApp};
use rfd::FileDialog;
use std::env::current_dir;

impl NesApp {
    pub fn file_open(&mut self) {
        let path = FileDialog::new()
            .add_filter("NES rom", &["nes"])
            .set_directory(&self.working_dir)
            .pick_file()
            .unwrap()
            .to_str()
            .unwrap()
            .to_owned();
        self.emu_tx.send(CtrlMSG::LoadProg(path));
        self.working_dir = current_dir().unwrap();
    }
}
