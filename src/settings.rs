use dotenv;
use serde::Deserialize;
use std::env;

#[derive(Eq, PartialEq, Debug, Deserialize)]
pub enum Mode {
    Development,
    Production,
}

#[derive(Debug, Deserialize)]
pub struct Settings {
    pub mode: Mode,
    pub cron: String,
    pub db_url: String,
}

impl Settings {
    pub fn new() -> Settings {
        match dotenv::dotenv().ok() {
            Some(path) => {
                println!("loaded env @ {}", path.to_string_lossy());
            }
            None => {
                println!("no .env file loaded");
            }
        }
        let mode = match env::var("MODE")
            .unwrap_or_else(|_| "development".into())
            .contains("dev")
        {
            true => Mode::Development,
            false => Mode::Production,
        };
        let cron = env::var("CRON").unwrap_or_else(|_| "0 0 0 * * *".into());
        let db_url = env::var("DATABASE_URL").unwrap_or("sqlite://speedy.db?mode=rwc".to_owned());
        Settings { mode, cron, db_url }
    }
}
