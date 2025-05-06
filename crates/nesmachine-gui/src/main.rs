mod gui;
mod playback_state;

use eframe::egui;
use egui::{CentralPanel, Frame, Ui, vec2};
use egui_tiles::{Behavior, LinearDir, SimplificationOptions, TileId, Tiles};
use egui_toast::Toasts;
use gui::*;
use nesmc_emu::NesMachine;
use playback_state::{PlaybackCommand, PlaybackState};
use poll_promise::Promise;
use std::time::Duration;
use web_time::Instant;

//#[cfg_attr(feature = "serde", derive(serde::Deserialize, serde::Serialize))]
#[derive(Debug)]
pub enum Pane {
    CpuBrowser(CpuBrowser),
    PpuBrowser(PpuBrowser),
    CpuInspector(CpuInspector),
    PpuInspector(PpuInspector),
    PpuNametableInspector(PpuNametableInspector),
    PpuPatternInspector(PpuPatternInspector),
    PlabackControl(PlaybackControl),
    Display(Display),
}

impl Pane {
    pub fn ui(&mut self, ui: &mut Ui, machine: &mut NesMachine, playback: &mut PlaybackState) {
        match self {
            Pane::CpuBrowser(pane) => pane.draw(ui, machine, playback),
            Pane::PpuBrowser(pane) => pane.draw(ui, machine),
            Pane::CpuInspector(pane) => pane.draw(ui, machine),
            Pane::PpuInspector(pane) => pane.draw(ui, machine),
            Pane::PpuNametableInspector(pane) => pane.draw(ui, machine),
            Pane::PpuPatternInspector(pane) => pane.draw(ui, machine),
            Pane::PlabackControl(pane) => pane.draw(ui, machine, playback),
            Pane::Display(pane) => pane.draw(ui, machine),
        }
    }

    pub fn title(&self) -> egui::WidgetText {
        match self {
            Pane::CpuBrowser(_) => "CPU Address Space".into(),
            Pane::PpuBrowser(_) => "PPU Address Space".into(),
            Pane::CpuInspector(_) => "CPU Inspector".into(),
            Pane::PpuInspector(_) => "PPU Inspector".into(),
            Pane::PpuNametableInspector(_) => "PPU Nametables".into(),
            Pane::PpuPatternInspector(_) => "PPU Patterns".into(),
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
    toasts: Toasts,

    open_file_fialog: Option<Promise<Option<Vec<u8>>>>,
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
        let ppu_nametable =
            tiles.insert_pane(Pane::PpuNametableInspector(PpuNametableInspector::default()));
        let ppu_pattern =
            tiles.insert_pane(Pane::PpuPatternInspector(PpuPatternInspector::default()));
        let display = tiles.insert_pane(Pane::Display(Display));

        let mut left_vertical =
            egui_tiles::Linear::new(LinearDir::Vertical, vec![playback, cpu_insp, ppu_insp]);
        left_vertical.shares.set_share(playback, 0.2);
        let left_vertical = tiles.insert_container(left_vertical);

        let graphics_inspectors = egui_tiles::Tabs::new(vec![ppu_nametable, ppu_pattern]);
        let graphics_inspectors = tiles.insert_container(graphics_inspectors);

        let main_top =
            egui_tiles::Linear::new(LinearDir::Horizontal, vec![display, graphics_inspectors]);
        let main_top = tiles.insert_container(main_top);

        let main_bottom = egui_tiles::Tabs::new(vec![cpu_browser, ppu_browser]);
        let main_bottom = tiles.insert_container(main_bottom);

        let main_vertical =
            egui_tiles::Linear::new(LinearDir::Vertical, vec![main_top, main_bottom]);
        let main_vertical = tiles.insert_container(main_vertical);

        let mut hbox =
            egui_tiles::Linear::new(LinearDir::Horizontal, vec![left_vertical, main_vertical]);
        hbox.shares.set_share(left_vertical, 0.25);
        let hbox = tiles.insert_container(hbox);

        tabs.push(hbox);

        let root = tiles.insert_tab_tile(tabs);
        let tree = egui_tiles::Tree::new("tile_tree", root, tiles);

        Self {
            tree,
            behavior: TreeBehavior::new(),
            toasts: Toasts::new(),
            open_file_fialog: None,
        }
    }
}

impl NesMachineApp {
    fn new(_cc: &eframe::CreationContext<'_>) -> Self {
        Self::default()
    }

    fn update_emu(&mut self) {
        let playback = &mut self.behavior.playback;
        let machine = &mut self.behavior.machine;

        if let Some(command) = &playback.command {
            match command {
                PlaybackCommand::Step => machine.step(),
                PlaybackCommand::Reset => machine.reset(),
                PlaybackCommand::Pause => playback.paused = true,
                PlaybackCommand::Unpause => {
                    playback.paused = false;
                    playback.t_next_frame = Instant::now();
                }
            }
            playback.command = None;
        }

        if !playback.paused {
            loop {
                // Run until machine reaches next frame
                loop {
                    machine.step();

                    if machine.ppu.cycle() == 0 && machine.ppu.scanline() == 0 {
                        break;
                    }
                    if playback.breakpoints.contains(&machine.cpu.pc) {
                        playback.paused = true;
                        break;
                    }
                }

                playback.t_next_frame += Duration::from_secs_f64(1. / 60.);

                if playback.paused {
                    break;
                }

                if Instant::now() < playback.t_next_frame {
                    break;
                }
            }
        }
    }
}

impl eframe::App for NesMachineApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        self.check_open_rom_dialog();
        self.consume_common_shortcuts(ctx);

        // GUI
        self.menu_bar(ctx);
        CentralPanel::default().frame(Frame::NONE).show(ctx, |ui| {
            self.tree.ui(&mut self.behavior, ui);
        });
        self.toasts.show(ctx);
        if !self.behavior.playback.paused {
            ctx.request_repaint();
        }

        // Emu logic
        self.update_emu();
    }
}

#[cfg(not(target_arch = "wasm32"))]
fn main() {
    let mut options = eframe::NativeOptions::default();
    options.viewport.inner_size = Some(vec2(1600., 1000.));
    let _ = eframe::run_native(
        "NesMachine",
        options,
        Box::new(|cc| Ok(Box::new(NesMachineApp::new(cc)))),
    );
}

#[cfg(target_arch = "wasm32")]
fn main() {
    use eframe::wasm_bindgen::JsCast as _;

    // Redirect `log` message to `console.log` and friends:
    eframe::WebLogger::init(log::LevelFilter::Debug).ok();

    let web_options = eframe::WebOptions::default();

    wasm_bindgen_futures::spawn_local(async {
        let document = web_sys::window()
            .expect("No window")
            .document()
            .expect("No document");

        let canvas = document
            .get_element_by_id("the_canvas_id")
            .expect("Failed to find the_canvas_id")
            .dyn_into::<web_sys::HtmlCanvasElement>()
            .expect("the_canvas_id was not a HtmlCanvasElement");

        let start_result = eframe::WebRunner::new()
            .start(
                canvas,
                web_options,
                Box::new(|cc| Ok(Box::new(NesMachineApp::new(cc)))),
            )
            .await;

        // Remove the loading text and spinner:
        if let Some(loading_text) = document.get_element_by_id("loading_text") {
            match start_result {
                Ok(_) => {
                    loading_text.remove();
                }
                Err(e) => {
                    loading_text.set_inner_html(
                        "<p> The app has crashed. See the developer console for details. </p>",
                    );
                    panic!("Failed to start eframe: {e:?}");
                }
            }
        }
    });
}
