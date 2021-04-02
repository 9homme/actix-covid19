use actix_web::dev::ServiceRequest;
use actix_web::Error;
use actix_web_httpauth::extractors::basic::{BasicAuth, Config};
use actix_web_httpauth::extractors::AuthenticationError;
use color_eyre::Result;

pub async fn basic_auth_validator(
    req: ServiceRequest,
    credentials: BasicAuth,
) -> Result<ServiceRequest, Error> {
    let config = req
        .app_data::<Config>()
        .map(|data| data.clone())
        .unwrap_or_else(Default::default);
    match validate_credentials(
        credentials.user_id(),
        credentials.password().unwrap().trim(),
    ) {
        Ok(_) => Ok(req),
        Err(_) => Err(AuthenticationError::from(config).into()),
    }
}

fn validate_credentials(user_id: &str, user_password: &str) -> Result<(), std::io::Error> {
    if user_id.eq("user") && user_password.eq("user123") {
        return Ok(());
    }
    return Err(std::io::Error::new(
        std::io::ErrorKind::Other,
        "Authentication failed!",
    ));
}
