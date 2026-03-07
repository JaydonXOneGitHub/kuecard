use kuecard_backend::abstractions::App;

use crate::helpers::{Config, CustomTheme};

pub struct MainApp {
    pub app: App,
    pub theme: CustomTheme,
    pub config: Config,
    pub scale_factor: f32,
}
