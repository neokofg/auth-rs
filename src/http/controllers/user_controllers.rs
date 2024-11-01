use actix_web::{web, HttpResponse, Responder};
use actix_web_httpauth::headers;
use actix_web_httpauth::headers::authorization::Bearer;
use serde_json::json;
use crate::services::user_service::UserService;

pub async fn get_user (
    user_service: web::Data<UserService>,
    token: web::Header<headers::authorization::Authorization<Bearer>>
) -> impl Responder {
    match user_service.get_user(token.as_ref().token()).await {
        Ok(response) => HttpResponse::Ok().json(response),
        Err(error) => HttpResponse::Unauthorized().json(json!({ "error": error })),
    }
}