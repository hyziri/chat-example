use serde::{Deserialize, Serialize};
use tower_sessions::Session;
use uuid::Uuid;

use crate::error::AppError;

pub const SESSION_USER_ID_KEY: &str = "chat_example:user:id";

#[derive(Default, Deserialize, Serialize, Debug)]
pub struct SessionUserId(pub String);

impl SessionUserId {
    pub async fn get(session: &Session) -> Result<Option<Uuid>, AppError> {
        session
            .get::<SessionUserId>(SESSION_USER_ID_KEY)
            .await?
            .map(|SessionUserId(id_str)| {
                id_str.parse::<Uuid>().map_err(|e| {
                    AppError::Internal(format!("Failed to parse session user id: {}", e))
                })
            })
            .transpose()
    }
}
