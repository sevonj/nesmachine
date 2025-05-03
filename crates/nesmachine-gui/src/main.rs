mod gui;
mod playback_state;

use eframe::egui;
use egui::{CentralPanel, Frame, Ui, vec2};
use egui_tiles::{Behavior, LinearDir, SimplificationOptions, TileId, Tiles};
use gui::*;
use nesmc_emu::NesMachine;
use playback_state::{PlaybackCommand, PlaybackState};

//#[cfg_attr(feature = "serde", derive(serde::Deserialize, serde::Serialize))]
#[derive(Debug)]
pub enum Pane {
    CpuBrowser(CpuBrowser),
    PpuBrowser(PpuBrowser),
    CpuInspector(CpuInspector),
    PpuInspector(PpuInspector),
    PlabackControl(PlaybackControl),
    Display(Display),
}

impl Pane {
    pub fn ui(&mut self, ui: &mut Ui, machine: &mut NesMachine, playback: &mut PlaybackState) {
        match self {
            Pane::CpuBrowser(mem_browser) => mem_browser.draw(ui, machine),
            Pane::PpuBrowser(ppu_browser) => ppu_browser.draw(ui, machine),
            Pane::CpuInspector(cpu_inspector) => cpu_inspector.draw(ui, machine),
            Pane::PpuInspector(ppu_inspector) => ppu_inspector.draw(ui, machine),
            Pane::PlabackControl(playback_control) => playback_control.draw(ui, machine, playback),
            Pane::Display(display) => display.draw(ui, machine),
        }
    }

    pub fn title(&self) -> egui::WidgetText {
        match self {
            Pane::CpuBrowser(_) => "CPU Address Space".into(),
            Pane::PpuBrowser(_) => "PPU Address Space".into(),
            Pane::CpuInspector(_) => "CPU Inspector".into(),
            Pane::PpuInspector(_) => "PPU Inspector".into(),
            Pane::PlabackControl(_) => "Playback".into(),
            Pane::Display(_) => "Display".into(),
        }
    }
}

pub struct TreeBehavior {
    machine: NesMachine,
    playback: PlaybackState,
}

impl TreeBehavior {
    fn new() -> Self {
        Self {
            machine: NesMachine::default(),
            playback: PlaybackState::default(),
        }
    }
}

impl Behavior<Pane> for TreeBehavior {
    fn pane_ui(
        &mut self,
        ui: &mut Ui,
        _tile_id: TileId,
        view: &mut Pane,
    ) -> egui_tiles::UiResponse {
        CentralPanel::default().show_inside(ui, |ui| {
            view.ui(ui, &mut self.machine, &mut self.playback);
        });
        egui_tiles::UiResponse::None
    }

    fn tab_title_for_pane(&mut self, view: &Pane) -> egui::WidgetText {
        view.title()
    }

    fn simplification_options(&self) -> SimplificationOptions {
        SimplificationOptions {
            all_panes_must_have_tabs: true,
            ..Default::default()
        }
    }

    fn gap_width(&self, _style: &egui::Style) -> f32 {
        2.
    }

    fn is_tab_closable(&self, _tiles: &Tiles<Pane>, _tile_id: TileId) -> bool {
        false
    }
}

struct NesMachineApp {
    tree: egui_tiles::Tree<Pane>,
    // #[cfg_attr(feature = "serde", serde(skip))]
    behavior: TreeBehavior,
}

impl Default for NesMachineApp {
    fn default() -> Self {
        let mut tiles = egui_tiles::Tiles::default();
        let mut tabs = vec![];

        let playback = tiles.insert_pane(Pane::PlabackControl(PlaybackControl));
        let cpu_insp = tiles.insert_pane(Pane::CpuInspector(CpuInspector));
        let ppu_insp = tiles.insert_pane(Pane::PpuInspector(PpuInspector));
        let cpu_browser = tiles.insert_pane(Pane::CpuBrowser(CpuBrowser::default()));
        let ppu_browser = tiles.insert_pane(Pane::PpuBrowser(PpuBrowser::default()));
        let display = tiles.insert_pane(Pane::Display(Display));

        let mut left_vertical =
            egui_tiles::Linear::new(LinearDir::Vertical, vec![playback, cpu_insp, ppu_insp]);
        left_vertical.shares.set_share(playback, 0.2);
        let left_vertical = tiles.insert_container(left_vertical);

        let browser_tabs = egui_tiles::Tabs::new(vec![cpu_browser, ppu_browser]);
        let browser_tabs = tiles.insert_container(browser_tabs);

        let center_vertical =
            egui_tiles::Linear::new(LinearDir::Vertical, vec![display, browser_tabs]);
        let center_vertical = tiles.insert_container(center_vertical);

        let mut hbox =
            egui_tiles::Linear::new(LinearDir::Horizontal, vec![left_vertical, center_vertical]);
        hbox.shares.set_share(left_vertical, 0.25);
        let hbox = tiles.insert_container(hbox);

        tabs.push(hbox);

        let root = tiles.insert_tab_tile(tabs);
        let tree = egui_tiles::Tree::new("tile_tree", root, tiles);

        Self {
            tree,
            behavior: TreeBehavior::new(),
        }
    }
}

impl NesMachineApp {
    fn new(_cc: &eframe::CreationContext<'_>) -> Self {
        Self::default()
    }

    fn update_emu(&mut self) {
        if let Some(command) = &self.behavior.playback.command {
            match command {
                PlaybackCommand::Step => self.behavior.machine.step(),
                PlaybackCommand::Reset => self.behavior.machine.reset(),
            }
            self.behavior.playback.command = None;
        }
    }
}

impl eframe::App for NesMachineApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        MenuBar::new(&mut self.behavior.machine).show(ctx);

        CentralPanel::default().frame(Frame::NONE).show(ctx, |ui| {
            self.tree.ui(&mut self.behavior, ui);
        });

        self.update_emu();
    }
}

fn main() {
    let mut options = eframe::NativeOptions::default();
    options.viewport.inner_size = Some(vec2(1600., 1000.));
    let _ = eframe::run_native(
        "NesMachine",
        options,
        Box::new(|cc| Ok(Box::new(NesMachineApp::new(cc)))),
    );
}
