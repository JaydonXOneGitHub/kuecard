use iced::{Event, Subscription, futures::Stream};

use kuecard_backend::message::Message;

use crate::{
    callbacks::start_ad_delivery,
    custommessage::CustomMessage,
    globals::{AD_REFRESH_INTERVAL_SECONDS, AD_URL},
    helpers::MainApp,
};

fn send_event(ev: Event) -> Message<CustomMessage> {
    return match &ev {
        Event::Keyboard(_) => Message::IcedEvent(ev),
        Event::Window(_) => Message::IcedEvent(ev),
        _ => Message::Nil,
    };
}

pub fn subscription(_: &MainApp) -> Subscription<Message<CustomMessage>> {
    return Subscription::batch(vec![
        iced::event::listen().map(send_event),
        Subscription::run(start_ad_delivery),
    ]);
}
