use super::Base;
use crate::{emulator::emu_debug::CtrlMSG, NesApp};
pub mod instruction_parser;
use eframe::emath::format_with_decimals_in_range;
use egui::{Button, Color32, Context, FontId, Frame, Layout, RichText, TextEdit, Ui};
use egui_extras::{Column, RetainedImage, TableBuilder};
use instruction_parser::*;
use num_traits::{clamp, ToPrimitive};

const FONT_TBL: FontId = FontId::monospace(12.0);
const FONT_TBLH: FontId = FontId::proportional(12.5);
const FONT_BUT: FontId = FontId::monospace(16.0);
const COL_TEXT: Color32 = Color32::DARK_GRAY;
const COL_TEXT_HI: Color32 = Color32::WHITE;

impl NesApp {
    pub fn emulator_toolbar(&mut self, ctx: &Context, ui: &mut Ui) {
        let text_onoff;
        match self.emu_running {
            true => text_onoff = RichText::new("⏼on/off").color(Color32::WHITE),
            false => text_onoff = RichText::new("⏼on/off"),
        }
        if ui.add(Button::new(text_onoff)).clicked() {
            self.emu_playing = false;
            self.emu_tx.send(CtrlMSG::PlaybackPlayPause(false));
            if self.emu_running {
                self.emu_running = false;
                self.emu_tx.send(CtrlMSG::PlaybackStop);
            } else {
                self.emu_running = true;
                self.emu_tx.send(CtrlMSG::PlaybackStart);
            }
        }

        ui.add_enabled_ui(self.emu_running, |ui| {
            let text_play;
            match self.emu_playing {
                true => text_play = RichText::new("⏸").color(Color32::WHITE),
                false => text_play = RichText::new("▶"),
            }
            if ui
                .add(Button::new(text_play).min_size(egui::vec2(24.0, 0.0)))
                .clicked()
            {
                self.emu_playing = !self.emu_playing;
                self.emu_tx
                    .send(CtrlMSG::PlaybackPlayPause(self.emu_playing));
            }
            ui.add_enabled_ui(!self.emu_playing, |ui| {
                if ui
                    .add(Button::new(RichText::new("|▶")).min_size(egui::vec2(24.0, 0.0)))
                    .clicked()
                {
                    self.emu_tx.send(CtrlMSG::PlaybackTick);
                }
            })
        });

        ui.separator();

        if ui.button("Reload").clicked() {
            self.emu_tx
                .send(CtrlMSG::LoadProg(self.current_prog.clone()));
        }

        ui.separator();
        if ui
            .selectable_label(self.emugui_display, "Show Graphics")
            .clicked()
        {
            self.emugui_display = !self.emugui_display;
        }
        ui.separator();
    }

    pub fn emulator_panel(&mut self, ctx: &Context, ui: &mut Ui) {
        self.refresh_emu_state();

        egui::CentralPanel::default().show(ctx, |ui| {
            egui::SidePanel::right("register_panel")
                .resizable(false)
                .show(ctx, |ui| {
                    self.stateview(ui);
                    self.regview(ui);
                });
            egui::CentralPanel::default().show(ctx, |ui| {
                if self.emugui_display {
                    egui::TopBottomPanel::top("display")
                        .resizable(true)
                        .show(ctx, |ui| {
                            self.display(ctx, ui);
                            self.emu_tx.send(CtrlMSG::GetDisp);
                        });
                }
                self.memview(ctx, ui);
            });
        });
    }

    fn memview(&mut self, ctx: &Context, ui: &mut Ui) {
        let width_adr: f32 = 96.0;
        let width_val: f32 = match self.mem_val_base == Base::Bin {
            true => 256.0,
            false => 96.0,
        };
        let width_ins: f32 = 192.0;

        egui::CentralPanel::default().show(ctx, |ui| {
            /*
             * Memview gives an illusion of scrolling through one large table that contains all
             * addresses, but it's size is actually always exactly what's visible on the screen.
             *
             * To keep the table always in view of the scroll area, we allocate the last known
             * scroll position worth of space before it.
             *
             * After adding the table, we allocate more space again to make the scrollarea total
             * height what the table would take if it contained every address.
             *
             * Table start offset is calculated from scroll position.
             */
            let row_height = 23.;
            let height = ui.available_height() - 30.;
            self.gui_memview_len = (height / row_height).to_usize().unwrap();
            let total_height = row_height * (self.emu_mem_len + 2) as f32 + 30.; // +2 because for some reason it fell short by that amount
            let view_height = height + 30.;
            self.gui_memview_scroll = egui::ScrollArea::vertical()
                .show(ui, |ui| {
                    ui.allocate_space(egui::Vec2 {
                        x: 0.,
                        y: self.gui_memview_scroll,
                    });
                    TableBuilder::new(ui)
                        .striped(true)
                        .auto_shrink([false; 2])
                        .max_scroll_height(f32::INFINITY)
                        .vscroll(false)
                        .column(Column::exact(width_adr)) // Address
                        .column(Column::exact(width_val)) // Value
                        .column(Column::exact(width_ins)) // Instruction
                        .column(Column::remainder()) // Registers PC/SP/FP
                        .header(20.0, |mut header| {
                            header.col(|ui| {
                                ui.heading(RichText::new("Address").font(FONT_TBLH.clone()));
                            });
                            header.col(|ui| {
                                ui.heading(RichText::new("Value").font(FONT_TBLH.clone()));
                            });
                            header.col(|ui| {
                                ui.heading(RichText::new("Instruction").font(FONT_TBLH.clone()));
                            });
                            header.col(|ui| {
                                ui.heading(RichText::new("").font(FONT_TBLH.clone()));
                            });
                        })
                        .body(|mut body| {
                            //let rowcount = self.emu_memory_len;
                            let mut op_vec: Vec<String> = Vec::new();
                            for i in 0..self.gui_memview_len {
                                if i >= self.gui_memview.len() {
                                    break;
                                }
                                let adr = match self.gui_memview_off as usize + i {
                                    0..=65535 => self.gui_memview_off + i as u16,
                                    _ => break,
                                };
                                let val = self.gui_memview[i];
                                let pc = self.emu_regs.pc;
                                let sp: u16 = self.emu_regs.sp as u16 + 0x100;
                                // Create strings
                                let mut reg_str = String::new();
                                if pc == adr || sp as u16 == adr {
                                    reg_str.push_str("<-- ");
                                    if pc == adr {
                                        reg_str.push_str("PC ")
                                    }
                                    if sp as u16 == adr {
                                        reg_str.push_str("SP ")
                                    }
                                }
                                let adr_str = match self.mem_adr_base {
                                    Base::Bin => format!("{adr:#b}"),
                                    Base::Dec => format!("{adr}"),
                                    Base::Hex => format!("{adr:#x}"),
                                };
                                let val_str = match self.mem_val_base {
                                    Base::Bin => format!("{val:#010b}"),
                                    Base::Dec => format!("{val}"),
                                    Base::Hex => format!("{val:#04x}"),
                                };
                                if pc == adr {
                                    op_vec.clear();
                                }
                                if op_vec.is_empty() {
                                    op_vec = instruction_to_string(val);
                                    if op_vec.is_empty() {
                                        panic!("Instruction to string returned empty!");
                                    }
                                }
                                let ins_str = Some(op_vec.remove(0)).unwrap();
                                // Decide style
                                let col = if adr == pc { COL_TEXT_HI } else { COL_TEXT };
                                body.row(20.0, |mut row| {
                                    row.col(|ui| {
                                        ui.label(
                                            RichText::new(adr_str)
                                                .font(FONT_TBL.clone())
                                                .color(col),
                                        );
                                    });
                                    row.col(|ui| {
                                        ui.label(
                                            RichText::new(val_str)
                                                .font(FONT_TBL.clone())
                                                .color(col),
                                        );
                                    });
                                    row.col(|ui| {
                                        ui.label(
                                            RichText::new(ins_str)
                                                .font(FONT_TBL.clone())
                                                .color(col),
                                        );
                                    });
                                    row.col(|ui| {
                                        ui.label(
                                            RichText::new(reg_str)
                                                .font(FONT_TBL.clone())
                                                .color(col),
                                        );
                                    });
                                });
                            }
                        });

                    ui.allocate_space(egui::Vec2 {
                        x: 0.,
                        y: total_height - self.gui_memview_scroll - view_height,
                    });
                    if self.emugui_follow_pc && self.emu_playing {
                        let pc_pos = row_height * self.emu_regs.pc as f32;
                        ui.scroll_to_rect(
                            egui::Rect {
                                min: egui::Pos2 {
                                    x: 0.,
                                    y: pc_pos - self.gui_memview_scroll,
                                },
                                max: egui::Pos2 {
                                    x: 0.,
                                    y: pc_pos - self.gui_memview_scroll + view_height,
                                },
                            },
                            Some(egui::Align::Center),
                        );
                    }
                })
                .state
                .offset
                .y;
            self.gui_memview_off = (self.gui_memview_scroll / row_height) as u16;
        });
        //    });
        //});
    }

    fn regview(&mut self, ui: &mut Ui) {
        // CPU Registers
        ui.label("CPU Registers");
        let reg_name_width: f32 = 48.0;
        let reg_val_width: f32 = match self.regs_base == Base::Bin {
            true => 256.0,
            false => 72.0,
        };
        TableBuilder::new(ui)
            .striped(true)
            .column(Column::exact(reg_name_width))
            .column(Column::exact(reg_val_width))
            .body(|mut body| {
                self.list_reg_u16(&mut body, "PC", self.emu_regs.pc);
                self.list_reg_u8(&mut body, "A", self.emu_regs.a);
                self.list_reg_u8(&mut body, "X", self.emu_regs.x);
                self.list_reg_u8(&mut body, "Y", self.emu_regs.y);
                self.list_reg_u16(&mut body, "STACK", self.emu_regs.sp as u16 + 0x100);
                self.reg_status(&mut body, "STATUS", self.emu_regs.status);
            });
        //if self.emu_halted {
        //    ui.label("HALT");
        //}
    }
    fn reg_status(&mut self, body: &mut egui_extras::TableBody, name: &str, val: u8) {
        let val_str = format!(
            "{}{}{}{}{}{}{}{}",
            if val & 0b1000_0000 > 0 { "N" } else { "-" },
            if val & 0b0100_0000 > 0 { "V" } else { "-" },
            if val & 0b0010_0000 > 0 { "U" } else { "-" },
            if val & 0b0001_0000 > 0 { "B" } else { "-" },
            if val & 0b0000_1000 > 0 { "D" } else { "-" },
            if val & 0b0000_0100 > 0 { "I" } else { "-" },
            if val & 0b0000_0010 > 0 { "Z" } else { "-" },
            if val & 0b0000_0001 > 0 { "C" } else { "-" },
        );
        body.row(20.0, |mut row| {
            row.col(|ui| {
                ui.label(name);
            });
            row.col(|ui| {
                ui.label(RichText::new(val_str).font(FONT_TBL.clone()));
            });
        });
    }
    fn list_reg_u8(&mut self, body: &mut egui_extras::TableBody, name: &str, val: u8) {
        let val_str = match self.regs_base {
            Base::Bin => format!("{val:#010b}"),
            Base::Dec => format!("{val}"),
            Base::Hex => format!("{val:#04x}"),
        };
        body.row(20.0, |mut row| {
            row.col(|ui| {
                ui.label(name);
            });
            row.col(|ui| {
                ui.label(val_str);
            });
        });
    }
    fn list_reg_u16(&mut self, body: &mut egui_extras::TableBody, name: &str, val: u16) {
        let val_str = match self.regs_base {
            Base::Bin => format!("{val:#018b}"),
            Base::Dec => format!("{val}"),
            Base::Hex => format!("{val:#06x}"),
        };
        body.row(20.0, |mut row| {
            row.col(|ui| {
                ui.label(name);
            });
            row.col(|ui| {
                ui.label(val_str);
            });
        });
    }

    fn ioview(&mut self, ctx: &Context, ui: &mut Ui) {
        ui.label("=CRT");
        // =CRT
        Frame::side_top_panel(&ctx.style())
            .fill(Color32::BLACK)
            .show(ui, |ui| {
                ui.label(self.buf_out.as_str());
                ui.allocate_space(egui::vec2(ui.available_width(), 0.0))
            });

        /*fn display_old(&mut self, ctx: &Context, ui: &mut Ui) {
        // Determine image size based on available w / h, whichever fits a smaller image
        let target_h = clamp(ui.available_height(), 120., 400.); // size limited for performance
        let target_w = clamp(ui.available_width(), 160., f32::INFINITY);
        let w;
        let h;
        if target_w > target_h * (160. / 120.) {
            w = (target_h * (160. / 120.)) as u32;
            h = target_h as u32;
        } else {
            w = target_w as u32;
            h = (target_w * (120. / 160.)) as u32;
        }
        ui.with_layout(Layout::top_down(egui::Align::Center), |ui| {
            self.emu_displaybuffer = Some(image::ImageBuffer::new(w, h));
            // This is a terribly inefficient way to make the image
            // TODO: figure out how to just rescale the original res pic.
            for (x, y, pixels) in self
                .emu_displaybuffer
                .as_mut()
                .unwrap()
                .enumerate_pixels_mut()
            {
                // px_off = px_x + px_y * 160
                let px_off = (x * 160 / w) + (y * 120 / h) * 160;
                *pixels = image::Rgba([
                    (self.emu_dispvec[px_off as usize] >> 4) as u8,
                    (self.emu_dispvec[px_off as usize]) as u8,
                    (self.emu_dispvec[px_off as usize] << 4) as u8,
                    255,
                ]);
            }
            let color_image = egui::ColorImage::from_rgba_unmultiplied(
                [w as usize, h as usize],
                &self.emu_displaybuffer.as_ref().unwrap(),
            );
            let render_result = RetainedImage::from_color_image("0.png", color_image);
            self.emu_displayimage = Some(render_result);
            if let Some(img) = &self.emu_displayimage {
                img.show(ui);
            }
        });*/
    }
    fn display(&mut self, ctx: &Context, ui: &mut Ui) {
        // Determine image size based on available w / h, whichever fits a smaller image
        let target_h = clamp(ui.available_height(), 240., 480.); // size limited for performance
        let target_w = clamp(ui.available_width(), 256., f32::INFINITY);
        let w;
        let h;
        if target_w > target_h * (256. / 240.) {
            w = (target_h * (256. / 240.)) as u32;
            h = target_h as u32;
        } else {
            w = target_w as u32;
            h = (target_w * (240. / 256.)) as u32;
        }
        ui.with_layout(Layout::top_down(egui::Align::Center), |ui| {
            self.emu_displaybuffer = Some(image::ImageBuffer::new(w, h));
            for (x, y, pixels) in self
                .emu_displaybuffer
                .as_mut()
                .unwrap()
                .enumerate_pixels_mut()
            {
                let px_off = (x * 256 / w) + (y * 240 / h) * 256;
                let color = self.emu_framebuffer[px_off as usize];
                let r = color & 0x03;
                let g = (color >> 2) & 0x03;
                let b = (color >> 4) & 0x03;
                *pixels = image::Rgba([
                    r.wrapping_mul(85),
                    g.wrapping_mul(85),
                    b.wrapping_mul(85),
                    255,
                ]);
            }
            let color_image = egui::ColorImage::from_rgba_unmultiplied(
                [w as usize, h as usize],
                &self.emu_displaybuffer.as_ref().unwrap(),
            );
            let render_result = RetainedImage::from_color_image("0.png", color_image);
            self.emu_displayimage = Some(render_result);
            if let Some(img) = &self.emu_displayimage {
                img.show(ui);
            }
        });
    }

    // Refresh cached regs and memory
    fn refresh_emu_state(&mut self) {
        self.emu_tx.send(CtrlMSG::GetState);
        self.emu_tx.send(CtrlMSG::GetRegs);
        self.emu_tx.send(CtrlMSG::GetMem(
            self.mem_use_ppu_space,
            self.gui_memview_off as usize..self.gui_memview_off as usize + self.gui_memview_len,
        ));
    }

    fn stateview(&mut self, ui: &mut Ui) {
        ui.label("Emulation speed:");
        ui.label(format_with_decimals_in_range(self.emu_achieved_speed as f64, 1..=1) + "%");
    }
}
