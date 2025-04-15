use egui::{Color32, DragValue, RichText, SidePanel, Ui};
use egui_extras::{Column, TableBuilder};
use nesmc_disassembler::{cpu_addresses::CpuAddressKind, instruction::DisassInst};
use nesmc_emu::bus::Bus;

use crate::components::ScrollSlider;

const W_TYPE_COL: f32 = 52.;
const W_ADDR_COL: f32 = 48.;
const W_VALUE_COL: f32 = 48.;
const H_HEADER: f32 = 24.;
const H_ROW: f32 = 16.;

const COL_ILLEGAL: Color32 = Color32::DARK_RED;

const MAX_ADDR: usize = 0xffff;

pub struct MemBrowser {
    /// Because of egui slider quirks, this is the offset from the **end**.
    /// 0xffff means the top, address 0x0.
    slider_pos: usize,
}

impl Default for MemBrowser {
    fn default() -> Self {
        Self {
            slider_pos: MAX_ADDR,
        }
    }
}

impl MemBrowser {
    pub fn draw(&mut self, ui: &mut Ui, bus: &mut Bus) {
        self.draw_sidebar(ui);

        ui.add(ScrollSlider::vertical(&mut self.slider_pos, 0..=MAX_ADDR));
        self.slider_pos = self.slider_pos.clamp(0, MAX_ADDR);

        let table_area_response = ui
            .vertical(|ui| {
                let available_h = ui.available_height();

                let tablebuider = TableBuilder::new(ui)
                    .column(Column::exact(W_TYPE_COL))
                    .column(Column::exact(W_ADDR_COL))
                    .column(Column::exact(W_VALUE_COL))
                    .column(Column::remainder())
                    .striped(true)
                    .vscroll(false);

                let table = tablebuider.header(H_HEADER, |mut header| {
                    header.col(|ui| {
                        ui.heading("Type");
                    });
                    header.col(|ui| {
                        ui.heading("Addr");
                    });
                    header.col(|ui| {
                        ui.heading("Value");
                    });
                    header.col(|ui| {
                        ui.heading("Disassembly");
                    });
                });

                table.body(|mut body| {
                    let mut addr = self.offset();
                    loop {
                        let mut bail = false;
                        let mem_value = bus.read(addr as u16);

                        body.row(H_ROW, |mut row| {
                            if addr > MAX_ADDR {
                                row.col(|_| {});
                                row.col(|_| {});
                                row.col(|_| {});
                                row.col(|ui| {
                                    if ui.next_widget_position().y >= available_h {
                                        bail = true;
                                    }
                                });
                                return;
                            }

                            row.col(|ui| {
                                let kind = CpuAddressKind::from(addr as u16);
                                let mut text = RichText::new(kind.short()).monospace();
                                if kind.is_mirror() {
                                    text = text.weak();
                                }

                                ui.label(text).on_hover_text(format!("{kind}"));
                            });

                            row.col(|ui| {
                                let text = format!("{addr:#06x}");

                                ui.label(RichText::new(text).monospace());
                            });

                            row.col(|ui| {
                                let text = format!("{mem_value:#04x}");

                                ui.label(RichText::new(text).monospace());
                            });

                            row.col(|ui| {
                                let disass = DisassInst::from_read_bus(bus, addr as u16);
                                let mut text = RichText::new(format!("{disass:?}")).monospace();
                                if disass.is_illegal() {
                                    text = text.color(COL_ILLEGAL);
                                }
                                ui.label(text);

                                if ui.next_widget_position().y >= available_h {
                                    bail = true;
                                }
                            });
                        });

                        if bail {
                            break;
                        }
                        addr += 1;
                    }
                });
            })
            .response;

        // scroll wheel
        if let (true, Some(_)) = (
            table_area_response.contains_pointer(),
            ui.input(|i| i.pointer.hover_pos()),
        ) {
            let scroll_delta = ui.input(|i| i.smooth_scroll_delta).y as isize;
            self.slider_pos = self.slider_pos.saturating_add_signed(scroll_delta);
        }
    }

    fn draw_sidebar(&mut self, ui: &mut Ui) {
        SidePanel::right("mem_browser_sidebar")
            .resizable(false)
            .show_inside(ui, |ui| {
                ui.label("Current Address:");
                let mut offset = self.offset();
                ui.add(
                    DragValue::new(&mut offset)
                        .hexadecimal(4, false, false)
                        .prefix("0x"),
                );
                self.jump_to(offset);

                ui.heading("Jump to");
                ui.horizontal(|ui| {
                    ui.monospace("0x0000");
                    if ui.link("Internal RAM").clicked() {
                        self.jump_to(0x0);
                    }
                });
                ui.horizontal(|ui| {
                    ui.monospace("0x2000");
                    if ui.link("PPU Registers").clicked() {
                        self.jump_to(0x2000);
                    }
                });
                ui.horizontal(|ui| {
                    ui.monospace("0x4000");
                    if ui.link("APU/IO Registers").clicked() {
                        self.jump_to(0x4000);
                    }
                });
                ui.horizontal(|ui| {
                    ui.monospace("0x4020");
                    if ui.link("Cart space").clicked() {
                        self.jump_to(0x4020);
                    }
                });
            });
    }

    fn offset(&self) -> usize {
        MAX_ADDR.saturating_sub(self.slider_pos)
    }

    fn jump_to(&mut self, offset: usize) {
        self.slider_pos = MAX_ADDR - offset;
    }
}
