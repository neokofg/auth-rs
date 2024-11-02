use sqlx::{Pool, Postgres};
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

    pub async fn get_user(&self, user: User) -> Result<UserResponse, String> {
        Ok(UserResponse {
            id: user.id,
            email: user.email
        })
    }
}