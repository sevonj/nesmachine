mod components;
mod ui_panes;

use eframe::egui;
use egui::{CentralPanel, Frame, Ui, vec2};
use egui_tiles::{Behavior, LinearDir, SimplificationOptions, TileId, Tiles};
use nesmc_emu::NesMachine;
use ui_panes::{CpuInspector, MemBrowser, MenuBar};

//#[cfg_attr(feature = "serde", derive(serde::Deserialize, serde::Serialize))]
#[derive(Debug)]
pub enum Pane {
    MemBrowser(MemBrowser),
    CpuInspector(CpuInspector),
}

impl Pane {
    pub fn ui(&mut self, ui: &mut Ui, machine: &mut NesMachine) {
        match self {
            Pane::MemBrowser(mem_browser) => mem_browser.draw(ui, machine),
            Pane::CpuInspector(cpu_inspector) => cpu_inspector.draw(ui, machine),
        }
    }

    pub fn title(&self) -> egui::WidgetText {
        match self {
            Pane::MemBrowser(_) => "Memory Browser".into(),
            Pane::CpuInspector(_) => "CPU Inspector".into(),
        }
    }
}

pub struct TreeBehavior {
    machine: NesMachine,
}

impl TreeBehavior {
    fn new() -> Self {
        Self {
            machine: NesMachine::default(),
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
        view.ui(ui, &mut self.machine);
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

        let cpu_insp = tiles.insert_pane(Pane::CpuInspector(CpuInspector));
        let mem_browser = tiles.insert_pane(Pane::MemBrowser(MemBrowser::default()));

        let mut hbox = egui_tiles::Linear::new(LinearDir::Horizontal, vec![cpu_insp, mem_browser]);
        hbox.shares.set_share(cpu_insp, 0.25);

        tabs.push(tiles.insert_container(hbox));

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
}

impl eframe::App for NesMachineApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        MenuBar::new(&mut self.behavior.machine).show(ctx);

        CentralPanel::default().frame(Frame::NONE).show(ctx, |ui| {
            self.tree.ui(&mut self.behavior, ui);
        });
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
