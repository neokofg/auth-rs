use crate::services::token_service::TokenService;

pub struct AuthMiddleware {
    token_service: TokenService
}

impl AuthMiddleware {
    pub fn new(token_service: TokenService) -> Self {
        Self { token_service}
    }
}