use crate::elements::uibutton::{AppTile, TextTile};

pub enum UIButton {
    AppTile(AppTile),
    TextTile(TextTile),
    Nil
}

impl UIButton {
    pub fn app_button(&self) -> Option<&AppTile> {
        return match self {
            Self::AppTile(at) => Option::Some(at),
            _ => Option::None
        };
    }

    pub fn text_button(&self) -> Option<&TextTile> {
        return match self {
            Self::TextTile(tt) => Option::Some(tt),
            _ => Option::None
        };
    }
}

impl UIButton {
    pub fn app_button_mut(&mut self) -> Option<&mut AppTile> {
        return match self {
            Self::AppTile(at) => Option::Some(at),
            _ => Option::None
        };
    }

    pub fn text_button_mut(&mut self) -> Option<&mut TextTile> {
        return match self {
            Self::TextTile(tt) => Option::Some(tt),
            _ => Option::None
        };
    }
}