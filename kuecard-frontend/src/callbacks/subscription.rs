use iced::Subscription;

use kuecard_backend::{
    message::Message
};

use crate::{
    custommessage::CustomMessage, 
    helpers::MainApp
};

pub fn subscription(_: &MainApp) -> Subscription<Message<CustomMessage>> {
    return iced::event::listen().map(Message::IcedEvent);
}