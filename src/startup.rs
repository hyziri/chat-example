use deadpool_postgres::{Config as DbConfig, Pool as DbPool, Runtime};
use tokio_postgres::NoTls;
use tower_sessions::{
    Expiry, SessionManagerLayer,
    cookie::{SameSite, time::Duration},
};
use tower_sessions_redis_store::{
    RedisStore,
    fred::prelude::{ClientLike, Config as RedisConfig, Pool as RedisPool},
};

use crate::{config::AppConfig, error::AppError};

pub async fn connect_to_database(config: &AppConfig) -> Result<DbPool, AppError> {
    let mut cfg = DbConfig::new();
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

pub async fn connect_to_session(
    config: &AppConfig,
) -> Result<SessionManagerLayer<RedisStore<RedisPool>>, AppError> {
    let config = RedisConfig::from_url(&config.redis_url())?;
    let pool = RedisPool::new(config, None, None, None, 6)?;

    pool.connect();
    pool.wait_for_connect().await?;

    let session_store = RedisStore::new(pool);

    // Set secure based on build mode: in development (debug) use false, otherwise true.
    let development_mode = cfg!(debug_assertions);
    let secure_cookies = !development_mode;

    let session = SessionManagerLayer::new(session_store)
        .with_secure(secure_cookies)
        .with_same_site(SameSite::Lax)
        .with_http_only(true)
        .with_expiry(Expiry::OnInactivity(Duration::days(7)));

    Ok(session)
}
