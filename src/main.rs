use actix_web::{web, App, HttpResponse, HttpServer};
use color_eyre::Result;
mod libs;

#[actix_web::main]
async fn main() -> Result<()> {
    HttpServer::new(|| {
        App::new()
            .route("/covid19", web::get().to(libs::handler::covid19))
            .route("/health", web::get().to(|| HttpResponse::Ok().body("Ok")))
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await?;

    Ok(())
}
