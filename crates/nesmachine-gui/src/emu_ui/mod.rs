mod mem_browser;

use egui::Context;
use mem_browser::MemBrowser;
use nesmc_emu::NesMachine;

#[derive(Default)]
pub struct EmuUi {
    mem_browser: MemBrowser,
}

impl EmuUi {
    pub fn draw(&mut self, ctx: &Context, machine: &mut NesMachine) {
        //egui::TopBottomPanel::bottom("bottom").show(ctx, |ui|{
        //    ui.allocate_space(vec2(5., 300.));
        //});
        egui::CentralPanel::default().show(ctx, |ui| {
            self.mem_browser.draw(ui, &mut machine.bus);
        });
    }
}
