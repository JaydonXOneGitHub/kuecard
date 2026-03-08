use iced::{Element, widget::column};
use kuecard_backend::message::Message;
use serde::{Deserialize, Serialize};

use crate::{custommessage::CustomMessage, helpers::AdImage};

#[derive(Clone, Serialize, Deserialize)]
pub struct AdMetadata {
    #[serde(rename = "content")]
    pub content: String,
    #[serde(rename = "adImage")]
    pub ad_image: AdImage,
}

impl AdMetadata {
    pub fn new(content: String, ad_image: AdImage) -> Self {
        return Self {
            content: content,
            ad_image: ad_image.into(),
        };
    }
}

impl AdMetadata {
    pub fn create_element<'a>(&self) -> Element<'a, Message<CustomMessage>> {
        return column![].into();
    }
}

impl Default for AdMetadata {
    fn default() -> Self {
        return Self {
            content: String::from("AD_TILE_PLACEHOLDER"),
            ad_image: AdImage::default(),
        };
    }
}
