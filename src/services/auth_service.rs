use argon2::Config;
use sqlx::{Pool, Postgres};
use tracing::{error, info, warn, Level};
use crate::entities::user::User;
use crate::http::requests::auth_requests::{LoginRequest, RegisterRequest};
use crate::http::responses::auth_responses::AuthResponse;
use crate::services::token_service::TokenService;
#[derive(Clone)]
pub struct AuthService {
    pool: Pool<Postgres>,
    token_service: TokenService,
}

impl AuthService {
    pub fn new(pool: Pool<Postgres>, token_service: TokenService) -> Self {
        Self { pool, token_service }
    }

    pub async fn register(&self, req: RegisterRequest) -> Result<AuthResponse, String> {
        let span = tracing::span!(Level::INFO, "register_user", email = %req.email);
        let _enter = span.enter();

        if let Ok(Some(_)) = sqlx::query!("SELECT id FROM users WHERE email = $1", req.email)
            .fetch_optional(&self.pool)
            .await
        {
            warn!("Попытка регистрации с существующим email");
            return Err("Email уже зарегестрирован".to_string());
        }

        info!("Создаем нового пользователя");
        let password_hash = self.hash_password(&req.password)?;

        match sqlx::query_as!(
            User,
            r#"
            INSERT INTO users (email, password_hash)
            VALUES ($1, $2)
            RETURNING id, email, password_hash, created_at, updated_at
            "#,
            req.email,
            password_hash
        )
            .fetch_one(&self.pool)
            .await
            {
                Ok(user) => {
                info!(user_id = %user.id, "Пользователь успешно создан");
                    let token = self.token_service
                        .create_token(user.id, "auth-token".to_string(), Some(30))
                        .await
                        .map_err(|_| "Ошибка при создании токена".to_string())?;
                Ok(AuthResponse {
                    token,
                })
                }
                Err(e) => {
                    error!(error = %e, "Ошибка при создании пользователя");
                    Err("Ошибка при создании пользователя ｡･ﾟﾟ*(>д<)*ﾟﾟ･｡".to_string())
                }
            }
    }

    pub async fn login(&self, req: LoginRequest) -> Result<AuthResponse, String> {
        let user = sqlx::query_as!(
            User,
            "SELECT * FROM users WHERE email = $1",
            req.email
        )
            .fetch_optional(&self.pool)
            .await
            .map_err(|_| "Ошибка при поиске пользователя".to_string())?
            .ok_or("Неверный email или пароль".to_string())?;

        if !self.verify_password(&req.password, &user.password_hash)? {
            return Err("Неверный email или пароль".to_string());
        }

        let token = self.token_service
            .create_token(user.id, "auth-token".to_string(), Some(30))
            .await
            .map_err(|_| "Ошибка при создании токена".to_string())?;

        Ok(AuthResponse {
            token
        })
    }

    pub async fn logout(&self, token: &str) -> Result<(), String> {
        let token_hash = self.token_service.hash_token(token);

        sqlx::query!(
            "DELETE FROM personal_access_tokens WHERE token = $1",
            token_hash
        )
            .execute(&self.pool)
            .await
            .map_err(|_| "Ошибка при удалении токена".to_string())?;

        Ok(())
    }

    fn hash_password(&self, password: &str) -> Result<String, String> {
        let salt = b"unique_salt_for_password";
        let config = Config::default();
        argon2::hash_encoded(password.as_bytes(), salt, &config)
            .map_err(|_| "Ошибка при хешировании пароля".to_string())
    }

    fn verify_password(&self, password: &str, hash: &str) -> Result<bool, String> {
        argon2::verify_encoded(hash, password.as_bytes())
            .map_err(|_| "Ошибка при проверке пароля".to_string())
    }
}