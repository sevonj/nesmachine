use egui::{
    Color32, DragValue, Label, RichText, Sense, SidePanel, Stroke, TextWrapMode, Ui, Vec2,
    epaint::CircleShape, vec2,
};
use egui_extras::{Column, TableBuilder};
use nesmc_disassembler::{cpu_addresses::CpuAddressKind, instruction::DisassInst};
use nesmc_emu::NesMachine;

use super::components::ScrollSlider;
use crate::playback_state::PlaybackState;

const W_TYPE_COL: f32 = 52.;
const W_ADDR_COL: f32 = 64.;
const W_VALUE_COL: f32 = 48.;
const H_HEADER: f32 = 24.;
const H_ROW: f32 = 16.;
const BREAKPOINT_COL: Color32 = Color32::from_rgb(178, 34, 34);

const MAX_ADDR: usize = 0xffff;

#[derive(Debug)]
pub struct CpuBrowser {
    /// Because of egui slider quirks, this is the offset from the **end**.
    /// 0xffff means the top, address 0x0.
    slider_pos: usize,
    follow_pc: bool,
}

impl Default for CpuBrowser {
    fn default() -> Self {
        Self {
            slider_pos: MAX_ADDR,
            follow_pc: false,
        }
    }
}

impl CpuBrowser {
    pub fn draw(&mut self, ui: &mut Ui, machine: &mut NesMachine, playback: &mut PlaybackState) {
        self.draw_sidebar(ui, machine);

        ui.add_enabled(
            !self.follow_pc,
            ScrollSlider::vertical(&mut self.slider_pos, 0..=MAX_ADDR, "cpu_browser_scrollbar"),
        );
        self.slider_pos = self.slider_pos.clamp(0, MAX_ADDR);

        let table_area_response = ui
            .vertical(|ui| {
                let max_h = ui.available_height() + ui.next_widget_position().y;

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
                    let pc = machine.cpu.pc as usize;
                    loop {
                        let mut bail = false;
                        let mem_value = machine.bus.read(addr as u16);

                        body.row(H_ROW, |mut row| {
                            row.set_selected(addr == pc);

                            if addr > MAX_ADDR {
                                row.col(|_| {});
                                row.col(|_| {});
                                row.col(|_| {});
                                row.col(|ui| {
                                    if ui.next_widget_position().y >= max_h {
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
                                ui.horizontal(|ui| {
                                    // Breakpoint dot
                                    let dot_diam = 8.0;
                                    let dot_size = Vec2::new(dot_diam, dot_diam);
                                    let dot_pos =
                                        ui.next_widget_position() + vec2(dot_diam / 2.0, 0.);
                                    let dot_resp = ui.allocate_exact_size(dot_size, Sense::all());

                                    let text = format!("{addr:#06x}");
                                    let text_resp = ui.label(RichText::new(text).monospace());

                                    let addr = addr as u16;

                                    if dot_resp.1.hovered() || text_resp.hovered() {
                                        let shape = CircleShape::stroke(
                                            dot_pos,
                                            dot_diam / 2.,
                                            Stroke::new(2., BREAKPOINT_COL),
                                        );
                                        ui.painter().add(shape);
                                    }
                                    if playback.breakpoints.contains(&addr) {
                                        let shape = CircleShape::filled(
                                            dot_pos,
                                            dot_diam / 2.,
                                            BREAKPOINT_COL,
                                        );
                                        ui.painter().add(shape);
                                    }

                                    if dot_resp.1.clicked() || text_resp.clicked() {
                                        if playback.breakpoints.contains(&addr) {
                                            playback.breakpoints.remove(&addr);
                                        } else {
                                            playback.breakpoints.insert(addr);
                                        }
                                    }
                                });
                            });

                            row.col(|ui| {
                                let text = format!("{mem_value:#04x}");

                                ui.label(RichText::new(text).monospace());
                            });

                            row.col(|ui| {
                                let disass = DisassInst::from_read_machine(machine, addr as u16);
                                let mut text = RichText::new(format!("{disass:?}")).monospace();
                                if disass.is_illegal() {
                                    text = text.weak();
                                }
                                ui.add(Label::new(text).wrap_mode(TextWrapMode::Truncate));

                                if ui.next_widget_position().y >= max_h {
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
        if !self.follow_pc {
            if let (true, Some(_)) = (
                table_area_response.contains_pointer(),
                ui.input(|i| i.pointer.hover_pos()),
            ) {
                let scroll_delta = ui.input(|i| i.raw_scroll_delta).y as isize / 2;
                self.slider_pos = self.slider_pos.saturating_add_signed(scroll_delta);
            }
        }
    }

    fn draw_sidebar(&mut self, ui: &mut Ui, machine: &NesMachine) {
        SidePanel::right("cpu_browser")
            .resizable(false)
            .show_inside(ui, |ui| {
                ui.checkbox(&mut self.follow_pc, "Follow PC");
                if self.follow_pc {
                    ui.disable();
                }

                ui.label("Current Address:");
                let mut offset = self.offset();
                ui.add(
                    DragValue::new(&mut offset)
                        .hexadecimal(4, false, false)
                        .prefix("0x"),
                );
                if !self.follow_pc {
                    self.jump_to(offset);
                } else {
                    self.jump_to(machine.cpu.pc as usize);
                }

                ui.separator();

                ui.heading("Jump to");

                ui.separator();

                ui.label("Memory Map");
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

                ui.separator();

                ui.label("CPU Registers");
                ui.horizontal(|ui| {
                    let text = format!("0x{:04x}", machine.cpu.pc);
                    ui.monospace(text);
                    if ui.link("PC").clicked() {
                        self.jump_to(machine.cpu.pc as usize);
                    }
                });
                ui.horizontal(|ui| {
                    let text = format!("0x{:04x}", machine.cpu.a);
                    ui.monospace(text);
                    if ui.link("A").clicked() {
                        self.jump_to(machine.cpu.a as usize);
                    }
                });
                ui.horizontal(|ui| {
                    let text = format!("0x{:04x}", machine.cpu.x);
                    ui.monospace(text);
                    if ui.link("X").clicked() {
                        self.jump_to(machine.cpu.x as usize);
                    }
                });
                ui.horizontal(|ui| {
                    let text = format!("0x{:04x}", machine.cpu.y);
                    ui.monospace(text);
                    if ui.link("Y").clicked() {
                        self.jump_to(machine.cpu.y as usize);
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
