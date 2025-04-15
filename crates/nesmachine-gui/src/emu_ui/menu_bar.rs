use egui::{Context, InnerResponse, TopBottomPanel, Ui, Widget};
use nesmc_emu::NesMachine;
use rfd::FileDialog;

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
            if ui.button("Quit").clicked() {
                ui.ctx().send_viewport_cmd(egui::ViewportCommand::Close);
                ui.close_menu();
            }

            if ui.button("Open").clicked() {
                if let Some(path) = FileDialog::new()
                    .add_filter("NES file", &["nes"])
                    .pick_file()
                {
                    let _ = self.machine.open(path);
                }
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
