#![allow(dead_code)]


use iced::{Size, window::Settings};

use crate::callbacks::{get_scale_factor, setup::initialize};



mod callbacks;
mod traits;
mod helpers;
mod custommessage;



fn main() -> Result<(), iced::Error> {
    let app = iced::application(
        initialize,
        callbacks::update, 
        callbacks::view
    );
    return app
    .subscription(callbacks::subscription)
    .scale_factor(get_scale_factor)
    .window(Settings {
        fullscreen: true,
        min_size: Option::Some(Size::new(1280.0, 720.0)),
        ..Default::default()
    })
    .resizable(false)
    .title("Kuecard")
    .run();
}
