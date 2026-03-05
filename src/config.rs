use std::any::type_name;
use std::str::FromStr;

use crate::error::AppError;

pub struct AppConfig {
    postgres_host: String,
    postgres_port: u16,
    postgres_db: String,
    postgres_user: String,
    postgres_password: String,
}

impl AppConfig {
    pub fn from_env() -> Result<Self, AppError> {
        Ok(Self {
            postgres_host: env_var("POSTGRES_HOST")?,
            postgres_port: env_var("POSTGRES_PORT")?,
            postgres_db: env_var("POSTGRES_DB")?,
            postgres_user: env_var("POSTGRES_USER")?,
            postgres_password: env_var("POSTGRES_PASSWORD")?,
        })
    }

    pub fn postgres_host(&self) -> &str {
        &self.postgres_host
    }

    pub fn postgres_port(&self) -> &u16 {
        &self.postgres_port
    }

    pub fn postgres_db(&self) -> &str {
        &self.postgres_db
    }

    pub fn postgres_user(&self) -> &str {
        &self.postgres_user
    }

    pub fn postgres_password(&self) -> &str {
        &self.postgres_password
    }
}

/// Gets env variable & parses to required type
///
/// # Returns
/// - Ok(<T>) -
/// - Err(AppError::MissingEnvVar) - The environment variable was not found in .env
/// - Err(AppError::InvalidEnvVar) - Environment variable is in an invalid format
fn env_var<T: FromStr>(var: &str) -> Result<T, AppError> {
    let value = std::env::var(var).map_err(|_| AppError::MissingEnvVar(var.to_string()))?;

    let parsed = value.parse::<T>().map_err(|_| AppError::InvalidEnvVar {
        var: var.to_string(),
        reason: format!("expected type {}", type_name::<T>()),
    })?;

    Ok(parsed)
}
