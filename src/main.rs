mod config;
mod controller;
mod data;
mod error;
mod model;
mod router;
mod service;
mod startup;

use tera::Tera;

use crate::{
    config::AppConfig, error::AppError, model::app::AppState, router::create_router,
    startup::connect_to_database,
};

#[tokio::main]
async fn main() -> Result<(), AppError> {
    dotenvy::dotenv()?;

    let config = AppConfig::from_env()?;

    // Initialize Tera templates
    let templates = Tera::new("templates/**/*.html").expect("Failed to parse templates");

    let pool = connect_to_database(config).await?;

    let state = AppState::new(pool, templates);
    let app = create_router().with_state(state);

    // Start the server
    let address = "127.0.0.1:8080";
    let listener = tokio::net::TcpListener::bind(address)
        .await
        .expect("Failed to bind to address");

    println!("Server running on http://{}", address);

    axum::serve(listener, app)
        .await
        .expect("Failed to start server");

    Ok(())
}
