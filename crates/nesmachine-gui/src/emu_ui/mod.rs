mod mem_browser;
mod menu_bar;

use egui::{CentralPanel, Context};
use mem_browser::MemBrowser;
use menu_bar::MenuBar;
use nesmc_emu::NesMachine;

#[derive(Default)]
pub struct EmuUi {
    mem_browser: MemBrowser,
}

impl EmuUi {
    pub fn draw(&mut self, ctx: &Context, machine: &mut NesMachine) {
        MenuBar::new(machine).show(ctx);

        CentralPanel::default().show(ctx, |ui| {
            self.mem_browser.draw(ui, &mut machine.bus);
        });
    }
}
