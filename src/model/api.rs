use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct ErrorDto {
    pub error: String,
}
