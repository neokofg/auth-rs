use actix_web::{web, HttpResponse, Responder};
use actix_web_httpauth::headers;
use actix_web_httpauth::headers::authorization::Bearer;
use serde_json::json;
use crate::http::requests::auth_requests::{LoginRequest, RegisterRequest};
use crate::services::auth_service::AuthService;

pub async fn register (
    auth_service: web::Data<AuthService>,
    req: web::Json<RegisterRequest>,
) -> impl Responder {
    match auth_service.register(req.into_inner()).await {
        Ok(response) => HttpResponse::Ok().json(response),
        Err(error) => HttpResponse::BadRequest().json(json!({ "error": error })),
    }
}

pub async fn login (
    auth_service: web::Data<AuthService>,
    req: web::Json<LoginRequest>,
) -> impl Responder {
    match auth_service.login(req.into_inner()).await {
        Ok(response) => HttpResponse::Ok().json(response),
        Err(error) => HttpResponse::Unauthorized().json(json!({ "error": error })),
    }
}

pub async fn logout (
    auth_service: web::Data<AuthService>,
    token: web::Header<headers::authorization::Authorization<Bearer>>,
) -> impl Responder {
    match auth_service.logout(token.as_ref().token()).await {
        Ok(response) => HttpResponse::Ok().json(response),
        Err(error) => HttpResponse::BadRequest().json(json!({ "error": error })),
    }
}