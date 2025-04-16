use egui::Ui;

use crate::playback_state::{PlaybackCommand, PlaybackState};

#[derive(Debug)]
pub struct PlaybackControl;

impl PlaybackControl {
    pub fn draw(&mut self, ui: &mut Ui, playback: &mut PlaybackState) {
        //CentralPanel::default().show_inside(ui, |ui| {
        ui.horizontal(|ui| {
            if !playback.running {
                ui.disable();
            }

            if ui.button("Reset").clicked() {
                playback.command = Some(PlaybackCommand::Reset);
            }

            if ui.button("Step").clicked() {
                playback.command = Some(PlaybackCommand::Step);
            }
        });
        //  });
    }
}
