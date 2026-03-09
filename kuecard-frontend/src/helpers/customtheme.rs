use iced::{Color, Shadow, Vector};

use crate::{globals::{BLUR_RADIUS, CONTAINER_SPACING}, helpers::Backdrop};

#[derive(Clone)]
pub struct CustomTheme {
    pub backdrop: Backdrop,
    pub button_backdrop: Backdrop,
    pub text_color: Color,
    pub sponsored_text_color: Option<Color>,
    pub unselected_color: Color,
    pub selected_color: Color,
    pub shadow: Shadow,
    pub font_name: Option<String>,
    pub font_size: u32
}

impl Default for CustomTheme {
    fn default() -> Self {
        return Self {
            backdrop: Backdrop::color(Color::from_rgba(
                0.1, 0.1, 0.1, 1.0
            )),
            button_backdrop: Backdrop::color(Color::BLACK),
            text_color: Color::WHITE,
            sponsored_text_color: Option::None,
            unselected_color: Color::from_rgba(
                0.2, 0.2, 0.2, 1.0
            ),
            selected_color: Color::WHITE,
            shadow: Shadow {
                color: Color::BLACK,
                offset: Vector::ZERO,
                blur_radius: BLUR_RADIUS + CONTAINER_SPACING * 1.5
            },
            font_name: Option::None,
            font_size: 26
        };
    }
}
