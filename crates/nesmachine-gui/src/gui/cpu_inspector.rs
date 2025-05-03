use egui::Ui;
use egui_extras::{Column, TableBuilder};
use nesmc_emu::NesMachine;

const H_HEADER: f32 = 24.;
const H_ROW: f32 = 16.;

#[derive(Debug)]
pub struct CpuInspector;

impl CpuInspector {
    pub fn draw(&mut self, ui: &mut Ui, machine: &mut NesMachine) {
        ui.label(format!("Cycles elapsed: {}", machine.cycle_count));

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
                    ui.monospace("A");
                });
                row.col(|ui| {
                    let text = format!("{:02X}", machine.cpu.a);
                    ui.monospace(text);
                });
            });
            body.row(H_ROW, |mut row| {
                row.col(|ui| {
                    ui.monospace("X");
                });
                row.col(|ui| {
                    let text = format!("{:02X}", machine.cpu.x);
                    ui.monospace(text);
                });
            });
            body.row(H_ROW, |mut row| {
                row.col(|ui| {
                    ui.monospace("Y");
                });
                row.col(|ui| {
                    let text = format!("{:02X}", machine.cpu.y);
                    ui.monospace(text);
                });
            });
            body.row(H_ROW, |mut row| {
                row.col(|ui| {
                    ui.monospace("PC");
                });
                row.col(|ui| {
                    let text = format!("{:04X}", machine.cpu.pc);
                    ui.monospace(text);
                });
            });
            body.row(H_ROW, |mut row| {
                row.col(|ui| {
                    ui.monospace("STACK");
                });
                row.col(|ui| {
                    let text = format!("{:04X}", machine.cpu.sp as usize + 0x100);
                    ui.monospace(text);
                });
            });
            body.row(H_ROW, |mut row| {
                row.col(|ui| {
                    ui.monospace("STATUS");
                });
                row.col(|ui| {
                    let text = format!("{}", machine.cpu.status);
                    ui.monospace(text);
                });
            });
        });
    }
}
