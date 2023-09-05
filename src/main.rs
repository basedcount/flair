use axum::http::StatusCode;
use axum::{routing::get, Router};

use clap::Parser;
use deadpool_sqlite::{Manager, Pool, Runtime};
use dotenv::dotenv;
use std::env;
use std::net::SocketAddr;

use crate::cli::Commands;

mod cli;
mod router;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    dotenv().ok();
    tracing_subscriber::fmt::init();

    let args = cli::Args::parse();
    match &args.command {
        Some(Commands::Serve) => {
            let flairs_port = env::var("FLAIRS_PORT").unwrap_or(String::from("6969"));
            if flairs_port.starts_with(":") {
                eprintln!("Please remove the ':' on your FLAIRS_PORT environment variable");
            }

            let port: u16 = flairs_port.parse().unwrap_or(6969);

            // Database setup
            let mut db_config = deadpool_sqlite::Config::new("flairs.db");
            let pool = db_config.create_pool(Runtime::Tokio1)?;
          

            let app = Router::new()
                .route("/", get(router::get_user_flair))
                .with_state(pool);

            let addr = SocketAddr::from(([127, 0, 0, 1], port));
            tracing::debug!("listening on {}", addr);
            axum::Server::bind(&addr)
                .serve(app.into_make_service())
                .await
                .unwrap();
        }
        None => std::process::exit(0),
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
