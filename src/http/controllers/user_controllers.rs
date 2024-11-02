use actix_web::{web, HttpResponse, Responder};
use serde_json::json;
use crate::middlewares::auth::AuthenticatedUser;
use crate::services::user_service::UserService;

pub async fn get_user (
    user_service: web::Data<UserService>,
    user: AuthenticatedUser
) -> impl Responder {
    match user_service.get_user(user.0).await {
        Ok(response) => HttpResponse::Ok().json(response),
        Err(error) => HttpResponse::Unauthorized().json(json!({ "error": error })),
    }
}