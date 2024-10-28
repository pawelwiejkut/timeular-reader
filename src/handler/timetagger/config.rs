use crate::config::Config;
use serde_derive::{Deserialize, Serialize};

const CONFIG_KEY: &str = "timetagger";

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct TimetaggerConfig{
    pub timetagger_url: String,
    pub api_key: String,
}

impl Default for TimetaggerConfig{
    fn default() -> Self {
        TimetaggerConfig{
            timetagger_url: String::new(),
            api_key: String::new(),
        }
    }
}

impl<'de> Config<'de> for TimetaggerConfig{}

pub fn create_config() -> TimetaggerConfig{
    crate::config::get_config::<TimetaggerConfig>(CONFIG_KEY)
}

pub fn update_config(config: &TimetaggerConfig) {
    crate::config::update_config(CONFIG_KEY, config);
}
