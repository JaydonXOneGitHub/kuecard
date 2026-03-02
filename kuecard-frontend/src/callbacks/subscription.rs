use iced::{Event, Subscription};

use kuecard_backend::message::Message;

use crate::{custommessage::CustomMessage, helpers::MainApp};

fn send_event(ev: Event) -> Message<CustomMessage> {
    return match &ev {
        Event::Keyboard(_) => Message::IcedEvent(ev),
        Event::Window(_) => Message::IcedEvent(ev),
        _ => Message::Nil,
    };
}

pub fn subscription(_: &MainApp) -> Subscription<Message<CustomMessage>> {
    return iced::event::listen().map(send_event);
}
