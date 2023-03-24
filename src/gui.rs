use crate::{emulator::emu_debug::CtrlMSG, NesApp};
use serde;
pub mod file_actions;
pub mod gui_emulator;

use egui::{Align, Button, DragValue, Layout, Modifiers};

#[derive(PartialEq, Default, serde::Deserialize, serde::Serialize)]
pub enum Base {
    Bin,
    #[default]
    Dec,
    Hex,
}

pub const SHORTCUT_NEW: egui::KeyboardShortcut =
    egui::KeyboardShortcut::new(Modifiers::COMMAND, egui::Key::N);
pub const SHORTCUT_OPEN: egui::KeyboardShortcut =
    egui::KeyboardShortcut::new(Modifiers::COMMAND, egui::Key::O);
pub const SHORTCUT_SAVE: egui::KeyboardShortcut =
    egui::KeyboardShortcut::new(Modifiers::COMMAND, egui::Key::S);
pub const SHORTCUT_SAVEAS: egui::KeyboardShortcut =
    egui::KeyboardShortcut::new(Modifiers::COMMAND.plus(Modifiers::SHIFT), egui::Key::S);

pub const SHORTCUT_GUI_EDIT: egui::KeyboardShortcut =
    egui::KeyboardShortcut::new(Modifiers::COMMAND, egui::Key::E);
pub const SHORTCUT_GUI_RUN: egui::KeyboardShortcut =
    egui::KeyboardShortcut::new(Modifiers::COMMAND, egui::Key::R);
pub const SHORTCUT_GUI_EMUGRAPHICS: egui::KeyboardShortcut =
    egui::KeyboardShortcut::new(Modifiers::COMMAND, egui::Key::G);
//pub const SHORTCUT_CLEAR: egui::KeyboardShortcut =
//    egui::KeyboardShortcut::new(Modifiers::COMMAND, egui::Key::E);
pub const SHORTCUT_COMPILE: egui::KeyboardShortcut =
    egui::KeyboardShortcut::new(Modifiers::COMMAND, egui::Key::B);

//pub const SHORTCUT_START: egui::KeyboardShortcut =
//    egui::KeyboardShortcut::new(Modifiers::NONE, egui::Key::Escape);
pub const SHORTCUT_STOP: egui::KeyboardShortcut =
    egui::KeyboardShortcut::new(Modifiers::NONE, egui::Key::Escape);
pub const SHORTCUT_TOGGLEPOWER: egui::KeyboardShortcut =
    egui::KeyboardShortcut::new(Modifiers::NONE, egui::Key::T);
pub const SHORTCUT_PLAY: egui::KeyboardShortcut =
    egui::KeyboardShortcut::new(Modifiers::NONE, egui::Key::Space);
pub const SHORTCUT_TICK: egui::KeyboardShortcut =
    egui::KeyboardShortcut::new(Modifiers::NONE, egui::Key::Enter);

pub const SHORTCUT_DEBUG_GUI: egui::KeyboardShortcut =
    egui::KeyboardShortcut::new(Modifiers::COMMAND.plus(Modifiers::ALT), egui::Key::D);

impl NesApp {
    pub fn gui_main(&mut self, ctx: &egui::Context) {
        egui::CentralPanel::default().show(ctx, |ui| {
            self.consume_shortcuts(ctx, ui);

            // Toolbar
            egui::TopBottomPanel::top("toolbar")
                .exact_height(32.0)
                .show(ctx, |ui| {
                    ui.with_layout(Layout::left_to_right(Align::Center), |ui| {
                        // File, Options, Help
                        self.gui_menubar_entries(ctx, ui);
                        ui.separator();
                        // Context toolbar
                        self.emulator_toolbar(ctx, ui);
                    });
                });
            egui::CentralPanel::default().show(ctx, |ui| {
                self.emulator_panel(ctx, ui);
            });
        });
    }

    fn gui_menubar_entries(&mut self, ctx: &egui::Context, ui: &mut egui::Ui) {
        ui.menu_button("File", |ui| {
            if ui
                .add(Button::new("Open").shortcut_text(ctx.format_shortcut(&SHORTCUT_OPEN)))
                .clicked()
            {
                self.file_open();
                ui.close_menu();
            }
        });

        ui.menu_button("Options", |ui| {
            ui.menu_button("Memory View", |ui| {
                ui.checkbox(&mut self.mem_use_ppu_space, "PPU Address space");
                ui.checkbox(&mut self.emugui_follow_pc, "Follow PC");
                ui.label("Memview Address base");
                ui.radio_value(&mut self.mem_adr_base, Base::Bin, "Binary");
                ui.radio_value(&mut self.mem_adr_base, Base::Dec, "Decimal");
                ui.radio_value(&mut self.mem_adr_base, Base::Hex, "Hex");
                ui.label("Memview Value base");
                ui.radio_value(&mut self.mem_val_base, Base::Bin, "Binary");
                ui.radio_value(&mut self.mem_val_base, Base::Dec, "Decimal");
                ui.radio_value(&mut self.mem_val_base, Base::Hex, "Hex");
                ui.label("Register Value base");
                ui.radio_value(&mut self.regs_base, Base::Bin, "Binary");
                ui.radio_value(&mut self.regs_base, Base::Dec, "Decimal");
                ui.radio_value(&mut self.regs_base, Base::Hex, "Hex");
            });

            ui.with_layout(Layout::left_to_right(Align::TOP), |ui| {
                ui.label("CPU Speed: ");
                ui.add_enabled(
                    !self.emu_turbo,
                    DragValue::new(&mut self.emu_speed)
                        .speed(0.1)
                        .clamp_range(1..=9999),
                );
                match self.emu_use_khz {
                    true => ui.label("KHz"),
                    false => ui.label("Hz"),
                }
            });
            ui.with_layout(Layout::left_to_right(Align::TOP), |ui| {
                ui.radio_value(&mut self.emu_use_khz, false, "Hz");
                if ui.radio_value(&mut self.emu_use_khz, true, "KHz").changed() {
                    if self.emu_use_khz {
                        self.emu_tx.send(CtrlMSG::SetRate(self.emu_speed * 1000.));
                    } else {
                        self.emu_tx.send(CtrlMSG::SetRate(self.emu_speed));
                    }
                };
            });
            if ui.checkbox(&mut self.emu_turbo, "Turbo Mode").changed() {
                self.emu_tx.send(CtrlMSG::SetTurbo(self.emu_turbo));
            };

            ui.menu_button("Language", |ui| {
                ui.add_enabled_ui(false, |ui| {
                    ui.label("no language support")
                    //ui.radio(true, "EN (English)");
                    //ui.radio(false, "FI (Suomi)");
                });
            });
        });

        ui.menu_button("Help", |ui| {
            if ui.button("↗TTK-91 Reference").clicked() {
                ui.output()
                    .open_url("https://www.cs.helsinki.fi/group/titokone/ttk91_ref_fi.html");
            }
        });
    }

    fn consume_shortcuts(&mut self, ctx: &egui::Context, ui: &mut egui::Ui) {
        if ui.input_mut().consume_shortcut(&SHORTCUT_DEBUG_GUI) {
            let debug = ui.style().debug.debug_on_hover;
            ui.ctx().set_debug_on_hover(!debug);
            println!("its debuggin time, {}", debug)
        }
        // General
        if ui.input_mut().consume_shortcut(&SHORTCUT_OPEN) {
            self.file_open()
        }
        // Emulator specific
        else {
            if ui.input_mut().consume_shortcut(&SHORTCUT_GUI_EMUGRAPHICS) {
                self.emugui_display = !self.emugui_display
            }
            if ui.input_mut().consume_shortcut(&SHORTCUT_TOGGLEPOWER) {
                match self.emu_running {
                    true => {
                        self.emu_tx.send(CtrlMSG::PlaybackStop);
                    }
                    false => {
                        self.emu_tx.send(CtrlMSG::PlaybackStart);
                    }
                }
            }
            if ui.input_mut().consume_shortcut(&SHORTCUT_STOP) {
                self.emu_tx.send(CtrlMSG::PlaybackStop);
            }
            if self.emu_running {
                if ui.input_mut().consume_shortcut(&SHORTCUT_PLAY) {
                    self.emu_tx
                        .send(CtrlMSG::PlaybackPlayPause(!self.emu_playing));
                }
                if ui.input_mut().consume_shortcut(&SHORTCUT_TICK) && !self.emu_playing {
                    self.emu_tx.send(CtrlMSG::PlaybackTick);
                    ctx.request_repaint_after(std::time::Duration::from_secs(1 / 60))
                }
            }
        }
    }
}
