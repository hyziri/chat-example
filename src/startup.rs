use deadpool_postgres::{Config, Pool, Runtime};
use tokio_postgres::NoTls;

use crate::{config::AppConfig, error::AppError};

pub async fn connect_to_database(config: AppConfig) -> Result<Pool, AppError> {
    let mut cfg = Config::new();
    cfg.host = Some(config.postgres_host().to_string());
    cfg.port = Some(*config.postgres_port());
    cfg.dbname = Some(config.postgres_db().to_string());
    cfg.user = Some(config.postgres_user().to_string());
    cfg.password = Some(config.postgres_password().to_string());

    let pool = cfg.create_pool(Some(Runtime::Tokio1), NoTls)?;

    // Check to ensure database configuration is valid
    let _ = pool.get().await?;

    Ok(pool)
}
