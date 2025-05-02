use egui::{DragValue, Response, RichText, SidePanel, Ui, Widget};
use egui_extras::{Column, TableBuilder};
use nesmc_disassembler::ppu_addresses::PpuAddressKind;
use nesmc_emu::NesMachine;

use super::components::ScrollSlider;

const W_TYPE_COL: f32 = 52.;
const W_ADDR_COL: f32 = 48.;
const H_HEADER: f32 = 24.;
const H_ROW: f32 = 16.;

const MAX_ADDR: usize = 0x3fff;

#[derive(Debug)]
pub struct PpuBrowser {
    /// Because of egui slider quirks, this is the offset from the **end**.
    /// 0xffff means the top, address 0x0.
    slider_pos: usize,
}

impl Default for PpuBrowser {
    fn default() -> Self {
        Self {
            slider_pos: MAX_ADDR,
        }
    }
}

impl PpuBrowser {
    pub fn draw(&mut self, ui: &mut Ui, machine: &mut NesMachine) {
        self.draw_sidebar(ui, machine);

        ui.add(ScrollSlider::vertical(
            &mut self.slider_pos,
            0..=MAX_ADDR,
            "ppu_browser_scrollbar",
        ));
        self.slider_pos = self.slider_pos.clamp(0, MAX_ADDR);

        let table_area_response = ui
            .vertical(|ui| {
                let max_h = ui.available_height() + ui.next_widget_position().y;

                let tablebuider = TableBuilder::new(ui)
                    .column(Column::exact(W_TYPE_COL))
                    .column(Column::exact(W_ADDR_COL))
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
                });

                table.body(|mut body| {
                    let mut addr = self.offset();
                    let pc = machine.cpu.pc as usize;
                    loop {
                        let mut bail = false;
                        let mem_value = machine.bus.read_ppu(addr as u16);
                        let kind = PpuAddressKind::from(addr as u16);
                        let is_mirror =
                            kind.is_mirror() || machine.bus.cart.is_ppu_addr_mirror(addr as u16);

                        body.row(H_ROW, |mut row| {
                            row.set_selected(addr == pc);

                            if addr > MAX_ADDR {
                                row.col(|_| {});
                                row.col(|_| {});
                                row.col(|ui| {
                                    if ui.next_widget_position().y >= max_h {
                                        bail = true;
                                    }
                                });
                                return;
                            }

                            row.col(|ui: &mut Ui| {
                                let mut text = RichText::new(kind.short()).monospace();
                                if is_mirror {
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
        if let (true, Some(_)) = (
            table_area_response.contains_pointer(),
            ui.input(|i| i.pointer.hover_pos()),
        ) {
            let scroll_delta = ui.input(|i| i.raw_scroll_delta).y as isize / 2;
            self.slider_pos = self.slider_pos.saturating_add_signed(scroll_delta);
        }
    }

    fn draw_sidebar(&mut self, ui: &mut Ui, machine: &NesMachine) {
        SidePanel::right("ppu_browser_sidebar")
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

                ui.separator();

                ui.heading("Jump to");

                ui.separator();

                ui.label("Memory Map");

                self.ppu_addr_link(machine, ui, 0x0000, "Pattern tbl 0");
                self.ppu_addr_link(machine, ui, 0x1000, "Pattern tbl 1");

                self.ppu_addr_link(machine, ui, 0x2000, "Nametable 0");
                self.ppu_addr_link(machine, ui, 0x2400, "Nametable 1");
                self.ppu_addr_link(machine, ui, 0x2800, "Nametable 2");
                self.ppu_addr_link(machine, ui, 0x2c00, "Nametable 3");

                self.ppu_addr_link(machine, ui, 0x3f00, "Palette indices");

                ui.separator();

                ui.label("PPU pointers");
                ui.horizontal(|ui| {
                    let text = format!("0x{:04x}", machine.bus.ppu_regs.ppu_addr);
                    ui.monospace(text);
                    if ui.link("VRAM Access Addr").clicked() {
                        self.jump_to(machine.bus.ppu_regs.ppu_addr as usize);
                    }
                });
                ui.horizontal(|ui| {
                    let text = format!("0x{:04x}", machine.bus.ppu_regs.oam_addr);
                    ui.monospace(text);
                    if ui.link("OAM Addr").clicked() {
                        self.jump_to(machine.bus.ppu_regs.oam_addr as usize);
                    }
                });
                ui.horizontal(|ui| {
                    let text = format!("0x{:04x}", machine.bus.ppu_regs.base_nametable_addr);
                    ui.monospace(text);
                    if ui.link("Current nametable").clicked() {
                        self.jump_to(machine.bus.ppu_regs.base_nametable_addr as usize);
                    }
                });
                ui.horizontal(|ui| {
                    let text = format!("0x{:04x}", machine.bus.ppu_regs.base_bg_pattern_addr);
                    ui.monospace(text);
                    if ui.link("BG pattern table").clicked() {
                        self.jump_to(machine.bus.ppu_regs.base_bg_pattern_addr as usize);
                    }
                });
                ui.horizontal(|ui| {
                    let text = format!("0x{:04x}", machine.bus.ppu_regs.base_sprite_pattern_addr);
                    ui.monospace(text);
                    if ui.link("Sprite pattern table").clicked() {
                        self.jump_to(machine.bus.ppu_regs.base_sprite_pattern_addr as usize);
                    }
                });
            });
    }

    fn ppu_addr_link(&mut self, machine: &NesMachine, ui: &mut Ui, addr: u16, text: &str) {
        ui.horizontal(|ui| {
            if ui.add(PpuAddrLink::new(machine, addr, text)).clicked() {
                self.jump_to(addr as usize);
            }
            if let Some(id) = machine
                .bus
                .cart
                .nt_arrangement()
                .and_then(|a| a.bank_id(addr))
            {
                ui.label(id);
            }
            if machine.bus.ppu_regs.base_bg_pattern_addr == addr {
                ui.label("(BG)");
            }
            if machine.bus.ppu_regs.base_sprite_pattern_addr == addr {
                ui.label("(Sprite)");
            }
            if machine.bus.ppu_regs.base_nametable_addr == addr {
                ui.label("(current)");
            }
        });
    }

    fn offset(&self) -> usize {
        MAX_ADDR.saturating_sub(self.slider_pos)
    }

    fn jump_to(&mut self, addr: usize) {
        self.slider_pos = MAX_ADDR - addr;
    }
}

pub struct PpuAddrLink<'a> {
    machine: &'a NesMachine,
    addr: u16,
    text: &'a str,
}

impl<'a> PpuAddrLink<'a> {
    pub fn new(machine: &'a NesMachine, addr: u16, text: &'a str) -> Self {
        Self {
            machine,
            addr,
            text,
        }
    }
}

impl Widget for PpuAddrLink<'_> {
    fn ui(self, ui: &mut Ui) -> Response {
        ui.horizontal(|ui| {
            let mut addr_text = RichText::new(format!("0x{:04X}", self.addr)).monospace();
            let mut title_text = RichText::new(self.text);
            if self.machine.bus.cart.is_ppu_addr_mirror(self.addr) {
                addr_text = addr_text.weak();
                title_text = title_text.weak();
            }
            ui.label(addr_text);
            ui.link(title_text)
        })
        .inner
    }
}
