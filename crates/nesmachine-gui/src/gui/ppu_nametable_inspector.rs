use egui::{
    Color32, Frame, Grid, Label, RichText, ScrollArea, Sense, Stroke, Ui, UiBuilder, Vec2, Widget,
    vec2,
};
use nesmc_emu::NesMachine;

#[derive(Debug)]
pub struct PpuNametableInspector {
    table_base_addr: usize,
    color_multiply: bool,
}
impl Default for PpuNametableInspector {
    fn default() -> Self {
        Self {
            table_base_addr: 0x2000,
            color_multiply: true,
        }
    }
}

impl PpuNametableInspector {
    pub fn draw(&mut self, ui: &mut Ui, machine: &mut NesMachine) {
        ScrollArea::vertical().show(ui, |ui| {
            ui.set_width(ui.available_width());

            ui.strong("Nametable Layout");

            Grid::new("nametable_select")
                .spacing(Vec2::ZERO)
                .show(ui, |ui| {
                    if ui
                        .add(NametableSelectCell::new(
                            0x2000,
                            machine,
                            self.table_base_addr == 0x2000,
                        ))
                        .clicked()
                    {
                        self.table_base_addr = 0x2000;
                    };
                    if ui
                        .add(NametableSelectCell::new(
                            0x2400,
                            machine,
                            self.table_base_addr == 0x2400,
                        ))
                        .clicked()
                    {
                        self.table_base_addr = 0x2400;
                    };
                    ui.end_row();
                    if ui
                        .add(NametableSelectCell::new(
                            0x2800,
                            machine,
                            self.table_base_addr == 0x2800,
                        ))
                        .clicked()
                    {
                        self.table_base_addr = 0x2800;
                    };
                    if ui
                        .add(NametableSelectCell::new(
                            0x2c00,
                            machine,
                            self.table_base_addr == 0x2c00,
                        ))
                        .clicked()
                    {
                        self.table_base_addr = 0x2c00;
                    };
                    ui.end_row();
                });
            ui.label("*currently in use");

            ui.strong(format!("${:04x}", self.table_base_addr));
            ui.checkbox(&mut self.color_multiply, "Color from value");

            Grid::new("nametable_content")
                .spacing(Vec2::ZERO)
                .min_col_width(8.)
                .show(ui, |ui| {
                    for y in 0..=29 {
                        ui.weak(format!("{:x} ", y * 32));
                        for x in 0..=31 {
                            let addr = self.table_base_addr + y * 32 + x;
                            ui.add(NametableCell::new(
                                addr as u16,
                                machine,
                                self.color_multiply,
                            ));
                        }
                        ui.end_row();
                    }
                });
        });
    }
}

#[derive(Debug)]
struct NametableSelectCell<'a> {
    addr: u16,
    machine: &'a NesMachine,
    selected: bool,
}

impl<'a> NametableSelectCell<'a> {
    pub fn new(addr: u16, machine: &'a NesMachine, selected: bool) -> Self {
        Self {
            addr,
            machine,
            selected,
        }
    }
}

impl Widget for NametableSelectCell<'_> {
    fn ui(self, ui: &mut Ui) -> egui::Response {
        let id = self
            .machine
            .bus
            .cart
            .nt_arrangement()
            .and_then(|a| a.bank_id(self.addr));

        let border = match id {
            Some("A") => Color32::LIGHT_BLUE,
            Some("B") => Color32::LIGHT_RED,
            Some("C") => Color32::LIGHT_GREEN,
            Some("D") => Color32::LIGHT_YELLOW,
            _ => Color32::GRAY,
        };
        let fill = ui.visuals().gray_out(border);

        ui.scope_builder(
            UiBuilder::new()
                .id_salt(format!("nametable_select_cell_{}", self.addr))
                .sense(Sense::all()),
            |ui| {
                let response = ui.response();

                Frame::default()
                    .fill(fill)
                    .stroke(Stroke::new(2., border))
                    .inner_margin(4)
                    .show(ui, |ui| {
                        ui.set_min_size(vec2(48., 32.));

                        ui.vertical(|ui| {
                            let mut addr_text =
                                RichText::new(format!("${:04X}", self.addr)).size(8.);
                            let mut id_text = RichText::new(format!(
                                "{}{}",
                                id.unwrap_or("N/A"),
                                if self.machine.bus.ppu_regs.ctrl.base_nametable_addr == self.addr {
                                    "*"
                                } else {
                                    ""
                                }
                            ))
                            .heading();

                            if self.selected {
                                addr_text = addr_text.color(Color32::WHITE).strong();
                                id_text = id_text.color(Color32::WHITE).strong();
                            }
                            if response.hovered() {
                                addr_text = addr_text.color(Color32::WHITE);
                                id_text = id_text.color(Color32::WHITE).underline();
                            }

                            ui.add(Label::new(addr_text).selectable(false));
                            ui.add(Label::new(id_text).selectable(false));
                        });
                    });
            },
        )
        .response
    }
}

#[derive(Debug)]
struct NametableCell<'a> {
    addr: u16,
    machine: &'a NesMachine,
    color_multiply: bool,
}

impl<'a> NametableCell<'a> {
    pub fn new(addr: u16, machine: &'a NesMachine, color_multiply: bool) -> Self {
        Self {
            addr,
            machine,
            color_multiply,
        }
    }
}

impl Widget for NametableCell<'_> {
    fn ui(self, ui: &mut Ui) -> egui::Response {
        let value = self.machine.bus.read_ppu(self.addr);
        let stroke_col = ui.visuals().faint_bg_color;

        Frame::default()
            .stroke(Stroke::new(1., stroke_col))
            .fill(Color32::BLACK)
            .show(ui, |ui| {
                let mut text = RichText::new(format!("{value:02x}")).monospace();
                if self.color_multiply {
                    text = text.color(Color32::WHITE.linear_multiply(value as f32 / 255.));
                }
                ui.label(text);
            })
            .response
    }
}
