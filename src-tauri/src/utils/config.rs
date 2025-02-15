use std::env;
use dotenv::dotenv;

pub struct Config {
    pub email: String,
    pub token: String,
    pub activation_code: String,
}

impl Config {
    pub fn load() -> Result<Self, String> {
        dotenv().ok();
        
        Ok(Self {
            email: env::var("TEST_EMAIL")
                .map_err(|_| "TEST_EMAIL not set in .env file".to_string())?,
            token: env::var("TEST_TOKEN")
                .map_err(|_| "TEST_TOKEN not set in .env file".to_string())?,
            activation_code: env::var("TEST_ACTIVATION_CODE")
                .map_err(|_| "TEST_ACTIVATION_CODE not set in .env file".to_string())?,
        })
    }
}
