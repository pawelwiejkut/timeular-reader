use async_trait::async_trait;
use chrono::{DateTime, Local};
use rand::{distributions::Alphanumeric, Rng};
use reqwest::Client;
use simplelog::info;
use serde_json::json;



use crate::{
    handler::timetagger::config::update_config,
    tracker::config::{Handler, Side},
};

use self::config::{create_config, TimetaggerConfig};

pub mod config;

#[derive(Debug, Default)]
pub struct Timetagger {
    client: Client,
    config: TimetaggerConfig,
}

#[async_trait]
impl Handler for Timetagger  {
    async fn handle(&self, side: &Side, duration: &(DateTime<Local>, DateTime<Local>)) {

        let key: String = rand::thread_rng()
            .sample_iter(&Alphanumeric)
            .take(8)  // Długość klucza
            .map(char::from)
            .collect();
        let t1 = duration.0.timestamp();
        let t2 = duration.1.timestamp();

        let body = json!([
            {
                "key": key,
                "mt": 1730093369,
                "t1": t1,
                "t2": t2,
                "ds": side.label,
                "st": 0.0
            }
        ]);


        info!(
            "Called Timetagger handler with side {side} and duration {:?}",
            duration
        );

        let response = self
            .client
            .put(format!("{}", self.config.timetagger_url.trim_end_matches('/'),))
            .header("Content-Type", "application/json")
            .header("authtoken", &self.config.api_key)
            .json(&body) 
            .send()
            .await;

            if response.is_err() {
                info!("API Error {}", response.unwrap_err());
                return;
            }
    
            info!("Response: {}", response.unwrap().text().await.unwrap());
        }
    }

pub async fn create_handler(setup: bool) -> Timetagger {
    let mut config = create_config();
    let client = Client::builder().build().unwrap();
    update_vendor_config(&mut config, setup);

    return Timetagger { client, config };
}

fn update_vendor_config(config: &mut TimetaggerConfig, setup: bool) {
    if setup || config.api_key.is_empty() {
        let mut api_key = String::new();
        let mut message =
            String::from_utf8("Provide your Timetagger api_key".as_bytes().to_vec()).unwrap();
        if config.api_key.is_empty() {
            message.push_str("\n leave blank to skip");
        }
        info!("{message}");

        std::io::stdin()
            .read_line(&mut api_key)
            .expect("Please provide api_key");
        api_key = api_key.trim().to_string();

        if !api_key.is_empty() {
            config.api_key = api_key;
            update_config(&config);
        }
    }

    if setup || config.timetagger_url.is_empty() {
        if setup || config.timetagger_url.is_empty() {
            let mut timetagger_url = String::new();
            let mut message =
                String::from_utf8("Provide your Timetagger URL".as_bytes().to_vec()).unwrap();
            if config.timetagger_url.is_empty() {
                message.push_str("\n leave blank to skip");
            }
            info!("{message}");
    
            std::io::stdin()
                .read_line(&mut timetagger_url)
                .expect("Please provide url");
            timetagger_url = timetagger_url.trim().to_string();
    
            if !timetagger_url.is_empty() {
                config.timetagger_url = timetagger_url;
                update_config(&config);
            }
        }
    }
}
