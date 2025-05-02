use egui::{ColorImage, Image, Rect, TextureFilter, TextureOptions, Ui, load::SizedTexture, vec2};
use nesmc_emu::NesMachine;

#[derive(Debug)]
pub struct Display;

impl Display {
    pub fn draw(&mut self, ui: &mut Ui, machine: &mut NesMachine) {
        let handle = ui.ctx().load_texture(
            "screen",
            ColorImage::from_rgb([256, 240], machine.ppu.framebuffer()),
            TextureOptions {
                magnification: TextureFilter::Nearest,
                ..Default::default()
            },
        );
        let image = Image::new(SizedTexture::new(handle.id(), vec2(256.0, 240.0)));

        const ASPECT_RATIO: f32 = 256.0 / 240.0;
        let mut size = ui.available_rect_before_wrap().size();
        if size.x / size.y > ASPECT_RATIO {
            size.x = size.y * ASPECT_RATIO;
        } else {
            size.y = size.x / ASPECT_RATIO;
        }
        let center = ui.available_rect_before_wrap().center();
        let rect = Rect::from_center_size(center, size);
        image.paint_at(ui, rect);
    }
}
