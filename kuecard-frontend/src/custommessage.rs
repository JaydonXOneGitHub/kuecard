use kuecard_backend::message::Message;

use crate::helpers::{AdMetadata, CustomTheme};

#[derive(Clone)]
pub enum CustomMessage {
    ThemeChanged(CustomTheme),
    AdLoaded(Result<AdMetadata, String>),
    Delay {
        time_in_milliseconds: u64,
        message: Box<Message<CustomMessage>>,
    },
    Exit,
    Nil,
}
