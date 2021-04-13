use serde::Deserialize;
use tracing::info;
use color_eyre::Result;
use dotenv::dotenv;
use tracing_subscriber::EnvFilter;
use eyre::WrapErr;

#[derive(Debug, Deserialize)]
pub struct Config {
    pub host: String,
    pub port: i32,
    pub mongo_host: String,
    pub mongo_port: i32,
    pub mongo_db: String,
    pub mongo_username: Option<String>,
    pub mongo_password: Option<String>
}

impl Config {

    pub fn from_env() -> Result<Config> {
        dotenv().ok();

        tracing_subscriber::fmt()
            .with_env_filter(EnvFilter::from_default_env())
            .init();

        info!("Loading configuration");

        let mut c = config::Config::new();

        c.merge(config::Environment::default())?;

        c.try_into()
            .context("loading configuration from environment")
    }
}
