use async_trait::async_trait;
use chrono::{DateTime, Local};
use derive_more::Display;
use log::debug;
use serde::{Deserialize, Serialize};

use crate::config::Config;

pub const ORIENTATION_CHARACTERISTIC_UUID: &str = "c7e70012-c847-11e6-8175-8c89a55d403c";
const CONFIG_KEY: &str = "timeular";

#[derive(Debug, Serialize, Deserialize)]
pub struct TimeularConfig {
    pub sides: Vec<Side>,
    pub handler: String,
}

#[derive(Debug, Serialize, Deserialize, Display, PartialEq)]
#[display(fmt = "{} {}", side_num, label)]
pub struct Side {
    pub side_num: u8,
    pub label: String,
    pub configurable: bool,
}

#[async_trait]
pub trait Handler: Sync + Send {
    async fn handle(self: &Self, side: &Side, duration: &(DateTime<Local>, DateTime<Local>)) {
        debug!("handler\n side: {:?}\n duration {:?}", side, duration)
    }
}
pub struct CallbackHandler {
    callback: fn(side: &Side, duration: &(DateTime<Local>, DateTime<Local>)),
}

#[async_trait]
impl Handler for CallbackHandler {
    async fn handle(self: &Self, side: &Side, duration: &(DateTime<Local>, DateTime<Local>)) {
        (self.callback)(side, duration);
    }
}

impl Default for TimeularConfig {
    fn default() -> Self {
        TimeularConfig {
            handler: String::new(),
            sides: vec![
                Side {
                    side_num: 1,
                    label: String::new(),
                    configurable: true,
                },
                Side {
                    side_num: 2,
                    label: String::new(),
                    configurable: true,
                },
                Side {
                    side_num: 3,
                    label: String::new(),
                    configurable: true,
                },
                Side {
                    side_num: 4,
                    label: String::new(),
                    configurable: true,
                },
                Side {
                    side_num: 5,
                    label: String::new(),
                    configurable: true,
                },
                Side {
                    side_num: 6,
                    label: String::new(),
                    configurable: true,
                },
                Side {
                    side_num: 7,
                    label: String::new(),
                    configurable: true,
                },
                Side {
                    side_num: 8,
                    label: String::new(),
                    configurable: true,
                },
                Side {
                    side_num: 9,
                    label: String::new(),
                    configurable: false,
                },
                Side {
                    side_num: 10,
                    label: String::new(),
                    configurable: false,
                },
                Side {
                    side_num: 11,
                    label: String::new(),
                    configurable: false,
                },
                Side {
                    side_num: 12,
                    label: String::new(),
                    configurable: false,
                },
                Side {
                    side_num: 13,
                    label: String::new(),
                    configurable: false,
                },
                Side {
                    side_num: 0,
                    label: String::new(),
                    configurable: false,
                },
            ],
        }
    }
}

impl<'de> Config<'de> for TimeularConfig {}

impl TimeularConfig {
    pub(crate) fn get_side(&self, side_num: &u8) -> &Side {
        self.find_side(side_num).unwrap()
    }

    pub fn is_trackable(&self, side_num: &u8) -> bool {
        self.find_side(side_num).is_some() && !self.find_side(side_num).unwrap().label.is_empty()
    }

    fn find_side(&self, side_num: &u8) -> Option<&Side> {
        self.sides.iter().find(|e| e.side_num.eq(side_num))
    }

    fn find_side_mut(&mut self, side_num: &u8) -> Option<&mut Side> {
        self.sides.iter_mut().find(|e| e.side_num.eq(side_num))
    }

    pub fn set_side(&mut self, side_num: u8, label: String) {
        if let Some(side) = self.find_side_mut(&side_num) {
            side.label = label;
        } else {
            self.sides.push(Side {
                side_num,
                label,
                configurable: true,
            });
        }
    }
}

pub fn get_timeular_config() -> TimeularConfig {
    crate::config::get_config::<TimeularConfig>(CONFIG_KEY)
}

pub fn update_timeular_config(config: &TimeularConfig) {
    crate::config::update_config(CONFIG_KEY, config);
}
