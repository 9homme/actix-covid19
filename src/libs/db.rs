use color_eyre::Result;
use mongodb::{options::ClientOptions, Client, Database};

use super::config::Config;

pub async fn get_db(config: &Config) -> Result<Database> {
    let client_options = ClientOptions::parse(
        format!("mongodb://{}:{}", config.mongo_host, config.mongo_port).as_str(),
    )
    .await?;
    let client = Client::with_options(client_options)?;
    let db = client.database(config.mongo_db.as_str());
    Ok(db)
}
