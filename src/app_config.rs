use serde::Deserialize;
use std::sync::Once;
use config::{Config, Environment};
use dotenv::dotenv;

#[derive(Deserialize, Debug)]
pub struct AppConfig {
    pub database_url: String,
    pub port: u16,
    // pub jwt_secret: String,
    // pub port: u16,
}

static INIT: Once = Once::new();
static mut CONFIG: Option<AppConfig> = None;

pub fn get_config() -> &'static AppConfig {
    unsafe {
        INIT.call_once(|| {
            dotenv().ok();
            let mut config = Config::new();
            config.merge(Environment::new()).unwrap();
            CONFIG = Some(config.try_into().unwrap());
        });
        CONFIG.as_ref().unwrap()
    }
}
