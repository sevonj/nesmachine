use egui_toast::{Toast, ToastKind};
use rfd::FileDialog;

use crate::NesMachineApp;

impl NesMachineApp {
    pub fn open_rom_dialog(&mut self) {
        if let Some(path) = FileDialog::new()
            .add_filter("NES file", &["nes"])
            .pick_file()
        {
            if let Err(e) = self.behavior.machine.open(path) {
                println!("{e}");
                self.toasts.add(Toast {
                    text: format!("{e}").into(),
                    kind: ToastKind::Error,
                    ..Default::default()
                });
            }
        }
    }
}
