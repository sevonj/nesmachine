use egui::Ui;
use nesmc_emu::NesMachine;

use crate::playback_state::{PlaybackCommand, PlaybackState};

#[derive(Debug)]
pub struct PlaybackControl;

impl PlaybackControl {
    pub fn draw(&mut self, ui: &mut Ui, machine: &mut NesMachine, playback: &mut PlaybackState) {
        ui.horizontal(|ui| {
            if ui.button("Reset").clicked() {
                playback.command = Some(PlaybackCommand::Reset);
            }

            if playback.paused {
                if ui.button("▶").clicked() {
                    playback.command = Some(PlaybackCommand::Unpause);
                }
            } else if ui.button("⏸").clicked() {
                playback.command = Some(PlaybackCommand::Pause);
            }

            if !playback.paused {
                ui.disable();
            }

            if ui.button("Step").clicked() {
                playback.command = Some(PlaybackCommand::Step);
            }

            if ui.button("Frame").clicked() {
                loop {
                    machine.step();
                    if machine.ppu.cycle() == 0 && machine.ppu.scanline() == 0 {
                        break;
                    }
                }
            }
        });
    }
}
