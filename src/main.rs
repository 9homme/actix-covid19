extern crate dotenv;

use actix_web::middleware::Logger;
use actix_web::{web, App, HttpResponse, HttpServer};
use color_eyre::Result;
use dotenv::dotenv;
use tracing::info;
use tracing_subscriber::EnvFilter;

mod libs;

#[actix_web::main]
async fn main() -> Result<()> {
    dotenv().ok();

    tracing_subscriber::fmt()
        .with_env_filter(EnvFilter::from_default_env())
        .init();

    info!("Starting application");

    HttpServer::new(|| {
        App::new()
            .wrap(Logger::default())
            .route("/covid19", web::get().to(libs::handler::covid19))
            .route("/health", web::get().to(|| HttpResponse::Ok().body("Ok")))
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await?;

    Ok(())
}
