use egui::Ui;
use egui_extras::{Column, TableBuilder};
use nesmc_emu::NesMachine;

const H_HEADER: f32 = 24.;
const H_ROW: f32 = 16.;

#[derive(Debug)]
pub struct PpuInspector;

impl PpuInspector {
    pub fn draw(&mut self, ui: &mut Ui, machine: &mut NesMachine) {
        //CentralPanel::default().show_inside(ui, |ui| {
        let tablebuider = TableBuilder::new(ui)
            .column(Column::auto())
            .column(Column::remainder())
            .striped(true);

        let table = tablebuider.header(H_HEADER, |mut header| {
            header.col(|ui| {
                ui.label("Register");
            });
            header.col(|ui| {
                ui.label("Value");
            });
        });

        table.body(|mut body| {
            body.row(H_ROW, |mut row| {
                row.col(|ui| {
                    ui.monospace("Scanline");
                });
                row.col(|ui| {
                    let text = format!("{}", machine.ppu.scanline());
                    ui.monospace(text);
                });
            });
            body.row(H_ROW, |mut row| {
                row.col(|ui| {
                    ui.monospace("Cycle");
                });
                row.col(|ui| {
                    let text = format!("{}", machine.ppu.cycle());
                    ui.monospace(text);
                });
            });
        });
        ui.horizontal(|ui| {
            ui.vertical(|ui| {
                ui.strong("Control register");
                ui.monospace(format!("{}", machine.bus.ppu_regs.ctrl));
                ui.checkbox(&mut machine.bus.ppu_regs.ctrl.nmi_enable, "NMI enable");
                ui.checkbox(&mut machine.bus.ppu_regs.ctrl.slave, "PPU slave");
                ui.checkbox(&mut machine.bus.ppu_regs.ctrl.tall_sprites, "Tall sprites");
                ui.label("BG pattern tbl");
                ui.horizontal(|ui| {
                    ui.selectable_value(
                        &mut machine.bus.ppu_regs.ctrl.base_bg_pattern_addr,
                        0x0000,
                        "0x0000",
                    );
                    ui.selectable_value(
                        &mut machine.bus.ppu_regs.ctrl.base_bg_pattern_addr,
                        0x1000,
                        "0x1000",
                    );
                });
                ui.label("Sprite pattern tbl");
                ui.horizontal(|ui| {
                    ui.selectable_value(
                        &mut machine.bus.ppu_regs.ctrl.base_sprite_pattern_addr,
                        0x0000,
                        "0x0000",
                    );
                    ui.selectable_value(
                        &mut machine.bus.ppu_regs.ctrl.base_sprite_pattern_addr,
                        0x1000,
                        "0x1000",
                    );
                });
                ui.label("VRAM address increment");
                ui.horizontal(|ui| {
                    ui.selectable_value(
                        &mut machine.bus.ppu_regs.ctrl.vram_big_increment,
                        false,
                        "1B",
                    );
                    ui.selectable_value(
                        &mut machine.bus.ppu_regs.ctrl.vram_big_increment,
                        true,
                        "32B",
                    );
                });
                ui.label("Nametable");
                ui.horizontal(|ui| {
                    ui.selectable_value(
                        &mut machine.bus.ppu_regs.ctrl.base_nametable_addr,
                        0x2000,
                        "0x2000",
                    );
                    ui.selectable_value(
                        &mut machine.bus.ppu_regs.ctrl.base_nametable_addr,
                        0x2400,
                        "0x2400",
                    );
                });
                ui.horizontal(|ui| {
                    ui.selectable_value(
                        &mut machine.bus.ppu_regs.ctrl.base_nametable_addr,
                        0x2800,
                        "0x2800",
                    );
                    ui.selectable_value(
                        &mut machine.bus.ppu_regs.ctrl.base_nametable_addr,
                        0x2c00,
                        "0x2c00",
                    );
                });
            });

            ui.separator();

            ui.vertical(|ui| {
                ui.strong("Mask register");
                ui.monospace(format!("{}", machine.bus.ppu_regs.mask));
                ui.checkbox(&mut machine.bus.ppu_regs.mask.blue, "Blue emphasis");
                ui.checkbox(&mut machine.bus.ppu_regs.mask.green, "Green emphasis");
                ui.checkbox(&mut machine.bus.ppu_regs.mask.red, "Red emphasis");
                ui.checkbox(&mut machine.bus.ppu_regs.mask.sprite, "Sprites enable");
                ui.checkbox(&mut machine.bus.ppu_regs.mask.bg, "BG enable");
                ui.checkbox(
                    &mut machine.bus.ppu_regs.mask.sprite_mask,
                    "Sprite mask show",
                );
                ui.checkbox(&mut machine.bus.ppu_regs.mask.bg_mask, "BG mask show");
                ui.checkbox(&mut machine.bus.ppu_regs.mask.grayscale, "Grayscale");

                ui.separator();

                ui.strong("Status register");
                ui.checkbox(&mut machine.bus.ppu_regs.vblank, "Vblank");
                ui.checkbox(&mut machine.bus.ppu_regs.sprite_0_hit, "Sprite 0 hit");
                ui.checkbox(&mut machine.bus.ppu_regs.sprite_overflow, "Sprite overflow");
            });
        });
    }
}
