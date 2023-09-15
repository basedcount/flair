use axum::http::StatusCode;
use axum::routing;
use axum::{routing::get, Router};

use clap::{Parser, Args};
use deadpool_sqlite::Runtime;
use dotenv::dotenv;
use std::env;
use std::net::SocketAddr;

use crate::cli::{init_db, Commands};

use ansi_term::Color;

mod cli;
mod router;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    dotenv().ok();
    tracing_subscriber::fmt::init();

    let args = cli::Args::parse();
    match &args.command {
        Some(Commands::Serve) => {
            let mut flairs_port = env::var("FLAIRS_PORT").unwrap_or(String::from("6969"));
            if flairs_port.starts_with(":") {
                eprintln!("Please remove the ':' on your FLAIRS_PORT environment variable");
                flairs_port = flairs_port.trim_start_matches(":").to_string();
            }

            let port: u16 = flairs_port.parse().unwrap_or(6969);

            // Database setup
            let db_config = deadpool_sqlite::Config::new("flairs.db");
            let pool = db_config.create_pool(Runtime::Tokio1)?;
            init_db(&pool).await?;

            let app = Router::new()
                .route("/api/v1/user", routing::put(router::put_user_flair))
                .with_state(pool);

            let addr = SocketAddr::from(([127, 0, 0, 1], port));
            tracing::debug!("listening on {}", addr);
            axum::Server::bind(&addr)
                .serve(app.into_make_service())
                .await
                .unwrap();
        },
        None => {}
    }

    Ok(())
}

/// Utility function for mapping any error into a `500 Internal Server Error`
/// response.
fn internal_error<E>(err: E) -> (StatusCode, String)
where
    E: std::error::Error,
{
    (StatusCode::INTERNAL_SERVER_ERROR, err.to_string())
}
