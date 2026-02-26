use kutamun::Direction;
use vector_x::Vector3;

use crate::message::Message;

#[derive(Clone)]
pub enum NavEvent {
    Navigate(Direction),
    Select(Vector3<usize>),
    Back,
    Nil
}

impl Into<Message> for NavEvent {
    fn into(self) -> Message {
        return Message::NavEvent(self);
    }
}