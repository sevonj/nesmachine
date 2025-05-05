use egui::{Button, Context, InnerResponse, TopBottomPanel, Ui};

use crate::NesMachineApp;

use super::keyboard_shortcuts::{SHORTCUT_OPEN, SHORTCUT_QUIT};

impl NesMachineApp {
    pub fn menu_bar(&mut self, ctx: &Context) -> InnerResponse<()> {
        TopBottomPanel::top("menu_bar").show(ctx, |ui| {
            ui.horizontal(|ui| {
                self.file_menu(ui);
                self.help_menu(ui);
            });
        })
    }

    fn file_menu(&mut self, ui: &mut Ui) -> egui::Response {
        ui.menu_button("File", |ui| {
            let open_button =
                Button::new("Open").shortcut_text(ui.ctx().format_shortcut(&SHORTCUT_OPEN));
            let quit_button =
                Button::new("Quit").shortcut_text(ui.ctx().format_shortcut(&SHORTCUT_QUIT));

            if ui.add(open_button).clicked() {
                self.open_rom_dialog();
                ui.close_menu();
            }

            ui.separator();

            if ui.add(quit_button).clicked() {
                ui.ctx().send_viewport_cmd(egui::ViewportCommand::Close);
                ui.close_menu();
            }
        })
        .response
    }

    fn help_menu(&mut self, ui: &mut Ui) -> egui::Response {
        ui.menu_button("Help", |ui| ui.label("NesMachine WIP"))
            .response
    }
}
