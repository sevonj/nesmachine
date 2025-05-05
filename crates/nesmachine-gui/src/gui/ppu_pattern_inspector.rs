use egui::{
    Color32, ColorImage, Frame, Grid, ScrollArea, Stroke, TextureFilter, TextureOptions, Ui, Vec2,
    Widget, load::SizedTexture, vec2,
};
use nesmc_emu::NesMachine;

#[derive(Debug, Default)]
pub struct PpuPatternInspector {}

impl PpuPatternInspector {
    pub fn draw(&mut self, ui: &mut Ui, machine: &mut NesMachine) {
        ScrollArea::vertical().show(ui, |ui| {
            ui.set_width(ui.available_width());

            ui.horizontal(|ui| {
                ui.vertical(|ui| {
                    ui.monospace("$0000 - Pattern table 0");
                    Grid::new("nametable_content")
                        .spacing(Vec2::ZERO)
                        .min_col_width(8.)
                        .show(ui, |ui| {
                            ui.weak(" ");
                            for x in 0..=15 {
                                ui.weak(format!("_{x:x} "));
                            }
                            ui.end_row();

                            for y in 0..=15 {
                                ui.weak(format!("{y:x} "));
                                for x in 0..=15 {
                                    let addr = x * 16 + y * 256;
                                    ui.add(PatternCell::new(addr as u16, machine));
                                }
                                ui.end_row();
                            }
                        });
                });
                ui.vertical(|ui| {
                    ui.monospace("$1000 - Pattern table 1");
                    Grid::new("nametable_content")
                        .spacing(Vec2::ZERO)
                        .min_col_width(8.)
                        .show(ui, |ui| {
                            ui.weak(" ");
                            for x in 0..=15 {
                                ui.weak(format!("_{x:x} "));
                            }
                            ui.end_row();

                            for y in 0..=15 {
                                ui.weak(format!("{y:x} "));
                                for x in 0..=15 {
                                    let addr = 0x1000 + x * 16 + y * 256;
                                    ui.add(PatternCell::new(addr as u16, machine));
                                }
                                ui.end_row();
                            }
                        });
                });
            });
        });
    }
}

#[derive(Debug)]
struct PatternCell<'a> {
    addr: u16,
    machine: &'a NesMachine,
}

impl<'a> PatternCell<'a> {
    pub fn new(addr: u16, machine: &'a NesMachine) -> Self {
        Self { addr, machine }
    }

    fn read_px(&self, x: usize, y: usize) -> (u8, u8, u8) {
        let x = x as u16 % 8;
        let y = y as u16 % 8;
        let bus = &self.machine.bus;

        let plane_0 = bus.read_ppu(self.addr + y);
        let plane_1 = bus.read_ppu(self.addr + y + 8);

        let val_lo = plane_0 >> (7 - x) & 1;
        let val_hi = plane_1 >> (7 - x) & 1;

        let color_idx = val_lo + (val_hi << 1);

        // Placeholder palette
        match color_idx {
            0 => (0, 0, 0),
            1 => (0x40, 0x40, 0x40),
            2 => (0x80, 0x80, 0x80),
            3 => (0xff, 0xff, 0xff),
            _ => unreachable!(),
        }
    }
}

impl Widget for PatternCell<'_> {
    fn ui(self, ui: &mut Ui) -> egui::Response {
        let stroke_col = ui.visuals().faint_bg_color;

        Frame::default()
            .stroke(Stroke::new(1., stroke_col))
            .fill(Color32::BLACK)
            .show(ui, |ui| {
                let mut tile = Vec::<u8>::with_capacity(64 * 3);

                for x in 0..=7 {
                    for y in 0..=7 {
                        let color = self.read_px(x, y);
                        tile.push(color.0);
                        tile.push(color.1);
                        tile.push(color.2);
                    }
                }

                let handle = ui.ctx().load_texture(
                    "screen",
                    ColorImage::from_rgb([8, 8], &tile),
                    TextureOptions {
                        magnification: TextureFilter::Nearest,
                        ..Default::default()
                    },
                );
                ui.image(SizedTexture::new(handle.id(), vec2(16.0, 16.0)));
            })
            .response
    }
}
