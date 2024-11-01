use rand::Rng;
use sqlx::{Pool, Postgres};
use uuid::Uuid;
use crate::entities::user::User;
use argon2::{self, Config};
use sqlx::types::time::OffsetDateTime;

#[derive(Clone)]
pub struct TokenService {
    pool: Pool<Postgres>
}

impl TokenService {
    pub fn new(pool: Pool<Postgres>) -> Self {
        Self { pool }
    }

    pub async fn create_token(&self, user_id: Uuid, name: String, expires_in_days: Option<i64>) -> Result<String, sqlx::Error> {
        let token = self.generate_token().await;
        let token_hash = self.hash_token(&token);

        let expires_at: Option<OffsetDateTime> = expires_in_days.map(|days| {
            OffsetDateTime::now_utc() + time::Duration::days(days)
        });

        sqlx::query!(
            r#"
            INSERT INTO personal_access_tokens (user_id, name, token, expires_at)
            VALUES ($1, $2, $3, $4)
            "#,
            user_id,
            name,
            token_hash,
            expires_at
        )
            .execute(&self.pool)
            .await?;

        Ok(token)
    }

    pub async fn validate_token(&self, token: &str) -> Result<Option<User>, sqlx::Error> {
        let token_hash = self.hash_token(token);

        let user = sqlx::query_as!(
            User,
            r#"
            SELECT u.* FROM users u
            INNER JOIN personal_access_tokens pat ON pat.user_id = u.id
            WHERE pat.token = $1
                AND (pat.expires_at IS NULL OR pat.expires_at > CURRENT_TIMESTAMP)
            "#,
            token_hash
        )
        .fetch_optional(&self.pool)
        .await?;

        if user.is_some() {
            sqlx::query!(
                r#"
                UPDATE personal_access_tokens
                SET last_used_at = CURRENT_TIMESTAMP
                WHERE token = $1
                "#,
                token_hash
            )
            .execute(&self.pool)
            .await?;
        }

        Ok(user)
    }

    pub async fn generate_token(&self) -> String {
        let mut rng = rand::thread_rng();
        let token: String = std::iter::repeat_with(|| rng.sample(rand::distributions::Alphanumeric))
            .take(40)
            .map(char::from)
            .collect();
        format!("{}_{}", Uuid::new_v4(), token)
    }

    pub fn hash_token(&self, token: &str) -> String {
        let salt = b"unique_salt_for_tokens";
        let config = Config::default();
        argon2::hash_encoded(token.as_bytes(), salt, &config).unwrap()
    }
}