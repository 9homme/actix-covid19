extern crate dotenv;

use actix_web::middleware::Logger;
use actix_web::{web, App, HttpResponse, HttpServer};
use actix_web_httpauth::middleware::HttpAuthentication;
use color_eyre::Result;
use tracing::info;
mod libs;

#[actix_web::main]
async fn main() -> Result<()> {
    let config = libs::config::Config::from_env().expect("Getting config from .env file");
    let db = libs::db::get_db(&config).await.expect("Connecting to mongodb");
    info!("Starting application");

    HttpServer::new(move || {
        App::new()
            .data(db.clone())
            .wrap(Logger::default())
            .service(
                web::scope("/app")
                    .wrap(HttpAuthentication::basic(libs::auth::basic_auth_validator))
                    .route("/covid19", web::get().to(libs::handler::covid19))
                    .route("/hash/{value}", web::get().to(libs::handler::hash))
                    .route("/user/add", web::post().to(libs::handler::add_user)),
            )
            .route("/health", web::get().to(|| HttpResponse::Ok().body("Ok")))
    })
    .bind(format!("{}:{}", config.host, config.port))?
    .run()
    .await?;

    Ok(())
}
