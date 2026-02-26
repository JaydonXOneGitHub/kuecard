use iced::{Color, Gradient};

use crate::helpers::Either;

#[derive(Clone)]
pub struct Backdrop {
    data: Either<Color, Gradient>
}

impl Backdrop {
    pub fn color(color: Color) -> Self {
        return Self {
            data: Either::A(color)
        };
    }

    pub fn gradient(gradient: Gradient) -> Self {
        return Self { 
            data: Either::B(gradient) 
        };
    }
}

impl Backdrop {
    pub fn to_background(&self) -> iced::Background {
        return match &self.data {
            Either::A(color) => {
                iced::Background::Color(color.clone())
            },
            Either::B(gradient) => {
                iced::Background::Gradient(gradient.clone())
            },
            Either::Neither => {
                iced::Background::Color(
                    Color::from_rgba(0.25, 0.25, 0.25, 1.0)
                )
            }
        };
    }
}

impl From<Color> for Backdrop {
    fn from(value: Color) -> Self {
        return Self::color(value);
    }
}

impl From<Gradient> for Backdrop {
    fn from(value: Gradient) -> Self {
        return Self::gradient(value);
    }
}