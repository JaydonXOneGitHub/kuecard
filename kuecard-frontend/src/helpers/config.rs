use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize)]
pub struct Config {
    #[serde(rename = "appDir")]
    pub app_dir: String,
    #[serde(rename = "backupAppDir")]
    pub backup_app_dir: String,
    #[serde(rename = "selectSFX")]
    pub select_sfx: Option<String>
}

impl Default for Config {
    fn default() -> Self {
        return Self {
            app_dir: "/home".into(),
            backup_app_dir: "/home".into(),
            select_sfx: Option::None
        };
    }
}
