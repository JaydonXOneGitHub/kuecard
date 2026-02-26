use iced::{Color, Shadow, Vector};

use crate::{callbacks::{BLUR_RADIUS, CONTAINER_SPACING}, helpers::Backdrop};

#[derive(Clone)]
pub struct CustomTheme {
    pub backdrop: Backdrop,
    pub button_backdrop: Backdrop,
    pub text_color: Color,
    pub unselected_color: Color,
    pub selected_color: Color,
    pub shadow: Shadow
}

impl Default for CustomTheme {
    fn default() -> Self {
        return Self {
            backdrop: Backdrop::color(Color::from_rgba(
                0.1, 0.1, 0.1, 1.0
            )),
            button_backdrop: Backdrop::color(Color::BLACK),
            text_color: Color::WHITE,
            unselected_color: Color::from_rgba(
                0.2, 0.2, 0.2, 1.0
            ),
            selected_color: Color::WHITE,
            shadow: Shadow {
                color: Color::BLACK,
                offset: Vector::ZERO,
                blur_radius: BLUR_RADIUS + CONTAINER_SPACING * 1.5
            }
        };
    }
}