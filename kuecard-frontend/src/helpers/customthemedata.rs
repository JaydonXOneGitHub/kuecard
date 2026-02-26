use iced::{Color, Shadow, Vector, gradient::Linear};
use serde::{Deserialize, Serialize};

use crate::{callbacks::{
    BLUR_RADIUS, 
    CONTAINER_SPACING
}, helpers::{Backdrop, CustomTheme}};



#[derive(Deserialize, Serialize, Clone)]
pub struct IcedColor {
    pub r: f32,
    pub g: f32,
    pub b: f32,
    pub a: f32
}

#[derive(Deserialize, Serialize, Clone)]
pub struct IcedGradient {
    pub start: IcedColor,
    pub end: IcedColor,
    pub angle: f32
}

#[derive(Deserialize, Serialize, Clone)]
pub struct IcedShadow {
    /// The color of the shadow.
    pub color: IcedColor,

    /// The offset of the shadow.
    pub offset: (f32, f32),

    /// The blur radius of the shadow.
    pub blur_radius: f32,
}



#[derive(Deserialize, Serialize, Clone)]
pub struct CustomThemeData {
    pub backdrop: IcedGradient,
    pub button_backdrop: IcedGradient,
    pub text_color: IcedColor,
    pub unselected_color: IcedColor,
    pub selected_color: IcedColor,
    pub shadow: IcedShadow
}

impl Default for CustomThemeData {
    fn default() -> Self {
        return Self {
            backdrop: IcedGradient {
                start: IcedColor { r: 0.1, g: 0.1, b: 0.1, a: 1.0 },
                end: IcedColor { r: 0.1, g: 0.1, b: 0.1, a: 1.0 },
                angle: 45.0
            },
            button_backdrop: IcedGradient {
                start: IcedColor { r: 0.0, g: 0.0, b: 0.0, a: 1.0 },
                end: IcedColor { r: 0.0, g: 0.0, b: 0.0, a: 1.0 },
                angle: 45.0
            },
            text_color: IcedColor {
                r: 1.0,
                g: 1.0,
                b: 1.0,
                a: 1.0
            },
            unselected_color: IcedColor {
                r: 0.2, g: 0.2, b: 0.2, a: 1.0
            },
            selected_color: IcedColor {
                r: 1.0,
                g: 1.0,
                b: 1.0,
                a: 1.0
            },
            shadow: IcedShadow {
                color: IcedColor {
                    r: 0.0,
                    g: 0.0,
                    b: 0.0,
                    a: 0.0
                },
                offset: (0.0, 0.0),
                blur_radius: BLUR_RADIUS + CONTAINER_SPACING * 1.5
            }
        };
    }
}

impl Into<CustomTheme> for CustomThemeData {
    fn into(self) -> CustomTheme {
        return CustomTheme {
            backdrop: Backdrop::gradient(
                iced::Gradient::Linear(
                    Linear::new(self.backdrop.angle)
                    .add_stop(0.0, Color {
                        r: self.backdrop.start.r,
                        g: self.backdrop.start.g,
                        b: self.backdrop.start.b,
                        a: self.backdrop.start.a,
                    })
                    .add_stop(1.0, Color {
                        r: self.backdrop.end.r,
                        g: self.backdrop.end.g,
                        b: self.backdrop.end.b,
                        a: self.backdrop.end.a,
                    })
                )
            ), 
            button_backdrop: Backdrop::gradient(
                iced::Gradient::Linear(
                    Linear::new(self.button_backdrop.angle)
                    .add_stop(0.0, Color {
                        r: self.button_backdrop.start.r,
                        g: self.button_backdrop.start.g,
                        b: self.button_backdrop.start.b,
                        a: self.button_backdrop.start.a,
                    })
                    .add_stop(1.0, Color {
                        r: self.button_backdrop.end.r,
                        g: self.button_backdrop.end.g,
                        b: self.button_backdrop.end.b,
                        a: self.button_backdrop.end.a,
                    })
                )
            ), 
            text_color: Color {
                r: self.text_color.r,
                g: self.text_color.g,
                b: self.text_color.b,
                a: self.text_color.a
            }, 
            unselected_color: Color {
                r: self.unselected_color.r,
                g: self.unselected_color.g,
                b: self.unselected_color.b,
                a: self.unselected_color.a
            }, 
            selected_color: Color {
                r: self.selected_color.r,
                g: self.selected_color.g,
                b: self.selected_color.b,
                a: self.selected_color.a
            }, 
            shadow: Shadow {
                color: Color {
                    r: self.shadow.color.r,
                    g: self.shadow.color.g,
                    b: self.shadow.color.b,
                    a: self.shadow.color.a
                },
                offset: Vector::new(
                    self.shadow.offset.0, 
                    self.shadow.offset.1
                ),
                blur_radius: self.shadow.blur_radius
            } 
        };
    }
}