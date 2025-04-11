mod components;
mod emu_ui;

use eframe::{Frame, egui};
use egui::Context;
use emu_ui::EmuUi;
use nesmc_emu::NesMachine;

fn main() {
    let native_options = eframe::NativeOptions::default();
    let _ = eframe::run_native(
        "NesMachine",
        native_options,
        Box::new(|cc| Ok(Box::new(NesMachineApp::new(cc)))),
    );
}

#[derive(Default)]
struct NesMachineApp {
    machine: NesMachine,
    emu_ui: EmuUi,
}

impl NesMachineApp {
    fn new(_cc: &eframe::CreationContext<'_>) -> Self {
        Self::default()
    }
}

impl eframe::App for NesMachineApp {
    fn update(&mut self, ctx: &Context, _frame: &mut Frame) {
        self.emu_ui.draw(ctx, &mut self.machine);
    }
}
