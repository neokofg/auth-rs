use moka::future::Cache;
use sqlx::{Pool, Postgres};
use core::time::Duration;
use uuid::Uuid;
use crate::entities::user::User;
use crate::http::responses::auth_responses::UserResponse;
use crate::services::token_service::TokenService;

#[derive(Clone)]
pub struct UserService {
    pool: Pool<Postgres>,
    token_service: TokenService,
}

impl UserService {
    pub fn new(pool: Pool<Postgres>, token_service: TokenService) -> Self {
        Self {
            pool,
            token_service,
        }
    }

    pub async fn get_user(&self, token: &str) -> Result<UserResponse, String> {
        let user = self.token_service
            .validate_token(token)
            .await
            .map_err(|_| "Ошибка при валидации токена".to_string())?
            .ok_or_else(|| "Токен недействителен".to_string())?;

        Ok(UserResponse {
            id: user.id,
            email: user.email
        })
    }
}