use crate::helpers::CustomTheme;

#[derive(Clone)]
pub enum CustomMessage {
    ThemeChanged(CustomTheme),
    Exit,
    Nil,
}
