//! Global, common shortcuts
//!

use egui::{Key, KeyboardShortcut as Shortcut, Modifiers, ViewportCommand};

use crate::NesMachineApp;

pub const SHORTCUT_OPEN: Shortcut = Shortcut::new(Modifiers::COMMAND, Key::O);
pub const SHORTCUT_QUIT: Shortcut = Shortcut::new(Modifiers::COMMAND, Key::Q);

impl NesMachineApp {
    pub fn consume_common_shortcuts(&mut self, ctx: &egui::Context) {
        if ctx.input_mut(|i| i.consume_shortcut(&SHORTCUT_QUIT)) {
            ctx.send_viewport_cmd(ViewportCommand::Close);
        }
        if ctx.input_mut(|i| i.consume_shortcut(&SHORTCUT_OPEN)) {
            self.open_rom_dialog();
        }
    }
}
