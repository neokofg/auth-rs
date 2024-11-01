use serde::{Deserialize, Serialize};
use time::OffsetDateTime;
use uuid::Uuid;

#[derive(Deserialize, Serialize, Debug, sqlx::FromRow)]
struct PersonalAccessToken {
    id: Uuid,
    user_id: Uuid,
    name: String,
    token: String,
    last_used_at: Option<OffsetDateTime>,
    created_at: Option<OffsetDateTime>,
    updated_at: OffsetDateTime,
}