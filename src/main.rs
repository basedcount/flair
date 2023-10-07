use axum::http::StatusCode;
use axum::routing;
use axum::Router;

use clap::Parser;
use deadpool_sqlite::Pool;
use deadpool_sqlite::Runtime;
use dotenv::dotenv;
use std::env;
use std::net::SocketAddr;

use crate::cli::{init_db, Commands};

mod cli;
mod db;
mod router;
mod verify;

#[derive(Clone)]
struct AppState {
    pool: Pool,
    lemmy_port: u16,
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    dotenv().ok();
    tracing_subscriber::fmt::init();

    let args = cli::Args::parse();
    match &args.command {
        Some(Commands::Serve) => {
            // Retrieve port where flair will run - defaults to 6969
            let mut flairs_port = env::var("FLAIRS_PORT").unwrap_or(String::from("6969"));
            if flairs_port.starts_with(":") {
                eprintln!("Please remove the ':' on your FLAIRS_PORT environment variable");
                flairs_port = flairs_port.trim_start_matches(":").to_string();
            }
            let port: u16 = flairs_port.parse().unwrap_or(6969);

            // Retrieve port where Lemmy is running - defaults to
            let mut lemmy_port_env = env::var("LEMMY_PORT").unwrap_or(String::from("8536"));
            if lemmy_port_env.starts_with(":") {
                eprintln!("Please remove the ':' on your LEMMY_PORT environment variable");
                lemmy_port_env = lemmy_port_env.trim_start_matches(":").to_string();
            }
            let lemmy_port: u16 = lemmy_port_env.parse().unwrap_or(8536);

            println!("The flair server is now running on port {}, polling a Lemmy instance on port {}!", port, lemmy_port);
            // Database setup
            let db_config = deadpool_sqlite::Config::new(
                env::var("FLAIR_DB_URL").unwrap_or(String::from("flairs.db")),
            );
            let pool = db_config.create_pool(Runtime::Tokio1)?;
            init_db(&pool).await?;

            let app_state = AppState { pool, lemmy_port };

            let app = Router::new()
                .route("/", routing::get(router::render_index))
                .route("/api/v1/user", routing::get(router::get_user_flair_api))   
                .route("/api/v1/user", routing::put(router::put_user_flair_api))
                .route("/api/v1/user", routing::delete(router::delete_user_api))
                .route("/api/v1/community", routing::get(router::get_community_flairs_api)) 
                .route("/api/v1/community", routing::put(router::put_community_flairs_api))
                .route("/api/v1/community", routing::delete(router::delete_community_flairs_api))
                .route("/api/v1/setup", routing::get(router::get_community_list_api))      
                .with_state(app_state);

            let addr = SocketAddr::from(([0, 0, 0, 0], port));
            tracing::debug!("listening on {}", addr);
            axum::Server::bind(&addr)
                .serve(app.into_make_service())
                .await
                .unwrap();
        }
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
    eprintln!("{}", err.to_string());
    (StatusCode::INTERNAL_SERVER_ERROR, err.to_string())
}
