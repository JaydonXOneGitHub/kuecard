use iced::advanced::graphics::core::Bytes;
use kuecard_backend::imagehandler::ImageHandle;
use reqwest::Response;
use serde::{Deserialize, Serialize};

use crate::callbacks::is_url_valid;

#[derive(Clone, Serialize, Deserialize)]
pub struct AdImage {
    #[serde(rename = "imageData")]
    pub pixel_data: Option<Vec<u8>>,
    #[serde(rename = "imageURL")]
    pub image_url: Option<String>,
    #[serde(rename = "width")]
    pub width: u32,
    #[serde(rename = "height")]
    pub height: u32,
    #[serde(skip)]
    pub handle: Option<ImageHandle>
}

impl AdImage {
    pub async fn new(
        image_url: impl Into<Option<String>>,
        pixel_data: impl Into<Option<Vec<u8>>>,
        width: u32,
        height: u32
    ) -> Self {
        let image_url: Option<String> = image_url.into();
        let pixel_data: Option<Vec<u8>> = load_image(pixel_data, &image_url).await;

        return Self {
            image_url: image_url,
            pixel_data: pixel_data,
            width: width,
            height: height,
            handle: Option::None
        };
    }
}

impl Default for AdImage {
    fn default() -> Self {
        return Self {
            image_url: Option::None,
            pixel_data: Option::None,
            width: 0,
            height: 0,
            handle: Option::None
        };
    }
}

impl AdImage {
    pub async fn load_image_bytes_from_url(&mut self) -> Result<(), String> {
        if let Option::None = &self.image_url {
            return Result::Err(String::from("Image URL does not exist!"));
        }

        let image_url: &String = self.image_url.as_ref().unwrap();

        let res: Result<Vec<u8>, String> = load_image_bytes(image_url).await;

        if res.is_err() {
            return Result::Err(res.err().unwrap());
        }

        self.pixel_data = Option::Some(res.ok().unwrap());

        let handle: ImageHandle = ImageHandle::from_bytes(match &self.pixel_data {
            Option::Some(pixel_data) => Bytes::from(pixel_data.clone()),
            Option::None => Bytes::new()
        });

        self.handle = Option::Some(handle);

        return Result::Ok(());
    }
}

pub async fn load_image_bytes(url: &String) -> Result<Vec<u8>, String> {
    if !is_url_valid(url) {
        return Result::Err(
            format!("URL (\"{}\") is not valid (requires prefix \"http://\" or \"https://\")", url)
        );
    }

    let res: reqwest::Result<Response> = reqwest::get(url).await;

    if res.is_err() {
        return Result::Err(res.err().unwrap().to_string());
    }

    let response: Response = res.unwrap();

    let res = response.bytes().await;

    if res.is_err() {
        return Result::Err(res.err().unwrap().to_string());
    }

    let bytes = res.unwrap();

    return Result::Ok(bytes.into());
}

async fn load_image(pixel_data: impl Into<Option<Vec<u8>>>, image_url: &Option<String>) -> Option<Vec<u8>> {
    match &image_url {
        Option::Some(s) => {
            let res: Result<Vec<u8>, String> = load_image_bytes(s).await;

            return match res {
                Result::Ok(bytes) => Option::Some(bytes),
                Result::Err(e) => {
                    println!("Reqwest error: {}", e);
                    Option::None
                }
            };
        },
        Option::None => { return pixel_data.into(); }
    }
}
