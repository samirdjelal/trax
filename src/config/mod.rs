use anyhow::Result;
use dotenv::dotenv;

pub struct Config {
    pub host: String,
    pub port: u16,
}
