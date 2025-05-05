use egui::{Button, Context, InnerResponse, TopBottomPanel, Ui, Widget};
use nesmc_emu::NesMachine;

use super::{
    dialogs::open_rom_dialog,
    keyboard_shortcuts::{SHORTCUT_OPEN, SHORTCUT_QUIT},
};

pub struct MenuBar<'a> {
    machine: &'a mut NesMachine,
}

impl<'a> MenuBar<'a> {
    pub fn new(machine: &'a mut NesMachine) -> Self {
        Self { machine }
    }

    pub fn show(self, ctx: &Context) -> InnerResponse<()> {
        TopBottomPanel::top("menu_bar").show(ctx, |ui| {
            ui.horizontal(|ui| {
                ui.add(FileMenu::new(self.machine));
                ui.add(HelpMenu::new());
            });
        })
    }
}

struct FileMenu<'a> {
    machine: &'a mut NesMachine,
}

impl<'a> FileMenu<'a> {
    pub fn new(machine: &'a mut NesMachine) -> Self {
        Self { machine }
    }
}

impl Widget for FileMenu<'_> {
    fn ui(self, ui: &mut Ui) -> egui::Response {
        ui.menu_button("File", |ui| {
            let open_button =
                Button::new("Open").shortcut_text(ui.ctx().format_shortcut(&SHORTCUT_OPEN));
            let quit_button =
                Button::new("Quit").shortcut_text(ui.ctx().format_shortcut(&SHORTCUT_QUIT));

            if ui.add(open_button).clicked() {
                open_rom_dialog(self.machine);
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
}

struct HelpMenu;

impl HelpMenu {
    pub fn new() -> Self {
        Self
    }
}

impl Widget for HelpMenu {
    fn ui(self, ui: &mut Ui) -> egui::Response {
        ui.menu_button("Help", |ui| ui.label("NesMachine WIP"))
            .response
    }
}
