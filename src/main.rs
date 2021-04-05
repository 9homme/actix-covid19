extern crate dotenv;

use actix_web::middleware::Logger;
use actix_web::{web, App, HttpResponse, HttpServer};
use actix_web_httpauth::middleware::HttpAuthentication;
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

    let db = libs::db::get_db().await.expect("Connecting to mongodb");
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
    .bind("127.0.0.1:8080")?
    .run()
    .await?;

    Ok(())
}
