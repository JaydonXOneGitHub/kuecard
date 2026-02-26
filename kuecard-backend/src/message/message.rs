use iced::Event;

use crate::{abstractions::ImageLoadList, message::NavEvent};

#[derive(Clone)]
pub enum Message<CustomMessage: Clone = ()> {
    IcedEvent(Event),
    PrintErr(String),
    NavEvent(NavEvent),
    LoadImageSet(ImageLoadList),
    ImagesLoaded(()),
    Custom(CustomMessage),
    Nil
}