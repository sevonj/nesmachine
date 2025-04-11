use std::ops::RangeInclusive;

use egui::{Button, Frame, Response, SidePanel, Slider, Ui, Vec2, Widget, style::HandleShape};

pub struct ScrollSlider<'a> {
    value: &'a mut usize,
    range: RangeInclusive<usize>,
}

const W_OUTER: f32 = 22.;
const BUTTON_SIZE: Vec2 = Vec2 { x: 16., y: 16. };

impl<'a> ScrollSlider<'a> {
    pub fn vertical(value: &'a mut usize, range: RangeInclusive<usize>) -> Self {
        Self { value, range }
    }
}

impl Widget for ScrollSlider<'_> {
    fn ui(self, ui: &mut Ui) -> Response {
        let response = SidePanel::right("scroll_slider")
            .exact_width(W_OUTER)
            .frame(Frame::NONE.inner_margin(2.))
            .resizable(false)
            .show_separator_line(false)
            .show_inside(ui, |ui| {
                ui.vertical_centered(|ui| {
                    ui.style_mut().spacing.button_padding = Vec2::ZERO;
                    ui.style_mut().spacing.slider_width = W_OUTER;

                    let resp_up = ui.add(Button::new("⏶").min_size(BUTTON_SIZE));

                    ui.style_mut().spacing.slider_width = ui.available_height() - W_OUTER;
                    let slider = Slider::new(self.value, self.range)
                        .show_value(false)
                        .vertical()
                        .handle_shape(HandleShape::Rect { aspect_ratio: 1.5 });
                    ui.add(slider);

                    let resp_dn = ui.add(Button::new("⏷").min_size(BUTTON_SIZE));

                    if resp_up.clicked() {
                        *self.value = self.value.saturating_add(2);
                    } else if resp_dn.clicked() {
                        *self.value = self.value.saturating_sub(2);
                    }
                })
            })
            .response;

        // scroll wheel
        if let (true, Some(_)) = (
            response.contains_pointer(),
            ui.input(|i| i.pointer.hover_pos()),
        ) {
            let scroll_delta = ui.input(|i| i.smooth_scroll_delta).y as isize;
            *self.value = self.value.saturating_add_signed(scroll_delta);
        }

        response
    }
}
