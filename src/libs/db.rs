use color_eyre::Result;
use mongodb::{options::ClientOptions, Client, Database};

pub async fn get_db() -> Result<Database> {
    let client_options = ClientOptions::parse("mongodb://localhost:27017").await?;
    let client = Client::with_options(client_options)?;
    let db = client.database("covid19");
    Ok(db)
}
