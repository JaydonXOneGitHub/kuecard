use kuecard_backend::elements::uibutton::AppTile;
use serde::{Deserialize, Serialize};

use crate::helpers::IcedColor;

#[derive(Deserialize, Serialize, Clone)]
pub struct AppTileData {
    #[serde(rename = "imgPath")]
    pub img_path: String,
    #[serde(rename = "altText")]
    pub alt_text: String,
    #[serde(rename = "command")]
    pub command: String,
    #[serde(rename = "tileColor")]
    pub color: IcedColor,
    #[serde(rename = "textColor")]
    pub text_color: IcedColor
}

impl Into<AppTile> for AppTileData {
    fn into(self) -> AppTile {
        return AppTile {
            img_path: self.img_path,
            alt_text: self.alt_text,
            command: self.command,
            color: iced::Color { 
                r: self.color.r, g: self.color.g, 
                b: self.color.b, a: self.color.a 
            },
            text_color: iced::Color { 
                r: self.text_color.r, g: self.text_color.g, 
                b: self.text_color.b, a: self.text_color.a 
            }
        };
    }   
}