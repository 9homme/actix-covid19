use super::model::User;
use super::repository::Repository;
use actix_web::Error;
use actix_web::{
    error::{ErrorBadRequest, ErrorUnauthorized},
    web::Data,
    FromRequest,
};
use actix_web_httpauth::extractors::basic::BasicAuth;
use blake2::{Blake2b, Digest};
use futures_util::future::ready;
use std::{borrow::Cow, future::Future, pin::Pin, sync::Arc};

#[derive(Debug)]
pub struct AuthenticatedUser {
    pub user: User,
}

impl FromRequest for AuthenticatedUser {
    type Error = Error;
    type Future = Pin<Box<dyn Future<Output = Result<Self, Self::Error>>>>;
    type Config = ();

    fn from_request(
        req: &actix_web::HttpRequest,
        payload: &mut actix_web::dev::Payload,
    ) -> Self::Future {
        let auth = BasicAuth::from_request(req, payload).into_inner();
        let repository = Data::<Arc<dyn Repository>>::from_request(req, payload).into_inner();

        match (auth, repository) {
            (Ok(basic_auth), Ok(repository)) => {
                let hash =
                    Blake2b::digest(basic_auth.password().unwrap_or(&Cow::from("")).as_bytes());
                let hashed_password_str = format!("{:x}", hash);
                let future = async move {
                    let user = repository.get_user(&basic_auth.user_id().to_string()).await;
                    match user {
                        Ok(Some(u)) if u.password_hash == hashed_password_str => {
                            Ok(AuthenticatedUser { user: u })
                        }
                        _ => Err(ErrorUnauthorized("Sorry, No luck!")),
                    }
                };
                Box::pin(future)
            }
            _ => Box::pin(ready(Err(ErrorBadRequest("Sorry, No luck!")))),
        }
        
    }
}
