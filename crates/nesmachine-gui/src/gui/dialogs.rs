use egui_toast::{Toast, ToastKind};
use poll_promise::Promise;
use rfd::AsyncFileDialog;

use crate::NesMachineApp;

impl NesMachineApp {
    #[cfg(not(target_arch = "wasm32"))]
    pub fn open_rom_dialog(&mut self) {
        if self.open_file_fialog.is_some() {
            return;
        }

        let promise = Promise::spawn_async(async {
            let f = AsyncFileDialog::new()
                .add_filter("NES file", &["nes"])
                .pick_file()
                .await;

            Some(f?.read().await)
        });

        self.open_file_fialog = Some(promise);
    }

    #[cfg(target_arch = "wasm32")]
    pub fn open_rom_dialog(&mut self) {
        if self.open_file_fialog.is_some() {
            return;
        }

        let promise = Promise::spawn_local(async {
            let f = AsyncFileDialog::new()
                .add_filter("NES file", &["nes"])
                .pick_file()
                .await;

            let Some(f) = f else {
                return None;
            };
            Some(f.read().await)
        });

        self.open_file_fialog = Some(promise);
    }

    pub fn check_open_rom_dialog(&mut self) {
        let Some(promise) = &mut self.open_file_fialog else {
            return;
        };

        let Some(result) = promise.ready() else {
            return;
        };

        if let Some(data) = result {
            if let Err(e) = self.behavior.machine.open_data(data) {
                println!("{e}");
                self.toasts.add(Toast {
                    text: format!("{e}").into(),
                    kind: ToastKind::Error,
                    ..Default::default()
                });
            }
        };

        self.open_file_fialog = None;
    }
}
