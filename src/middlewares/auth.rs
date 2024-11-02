use actix_web::dev::Payload;
use actix_web::{web, Error, FromRequest, HttpRequest};
use std::future::Future;
use std::pin::Pin;
use actix_web::error::{ErrorInternalServerError, ErrorUnauthorized};
use serde::Serialize;
use crate::entities::user::User;
use crate::services::token_service::TokenService;
#[derive(Serialize)]
pub struct AuthenticatedUser(pub User);

impl FromRequest for AuthenticatedUser {
    type Error = Error;
    type Future = Pin<Box<dyn Future<Output = Result<Self, Self::Error>>>>;

    fn from_request(req: &HttpRequest, _: &mut Payload) -> Self::Future {
        // Клонируем всё необходимое до async блока
        let token_service = req.app_data::<web::Data<TokenService>>().cloned();
        let auth_header = req.headers()
            .get("Authorization")
            .and_then(|h| h.to_str().ok())
            .map(|s| s.to_owned());

        Box::pin(async move {
            let token_service = token_service.ok_or_else(|| {
                ErrorInternalServerError("Token service not configured")
            })?;

            let auth_str = auth_header.ok_or_else(|| {
                ErrorUnauthorized("No authorization header")
            })?;

            let token = auth_str.strip_prefix("Bearer ")
                .ok_or_else(|| ErrorUnauthorized("Invalid token format"))?;

            let user = token_service.validate_token(token).await.unwrap()
                .ok_or_else(|| ErrorUnauthorized("Invalid token"))?;

            Ok(AuthenticatedUser(user))
        })
    }
}