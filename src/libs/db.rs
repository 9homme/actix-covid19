use color_eyre::Result;
use mongodb::{options::ClientOptions, Client, Database};

use super::config::Config;

pub async fn get_db(config: &Config) -> Result<Database> {
    let username = config.mongo_username.as_ref();
    let password = config.mongo_password.as_ref();
    let connection_url = if username.is_some() {
        format!(
            "mongodb://{}:{}@{}:{}",
            username.unwrap_or(&"".to_string()),
            password.unwrap_or(&"".to_string()),
            config.mongo_host,
            config.mongo_port
        )
    } else {
        format!(
            "mongodb://{}:{}",
            config.mongo_host,
            config.mongo_port
        )
    };
    let client_options = ClientOptions::parse(&connection_url)
    .await?;

    let client = Client::with_options(client_options)?;
    let db = client.database(config.mongo_db.as_str());

    Ok(db)
}
