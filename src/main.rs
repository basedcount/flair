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
    docker: bool,
    lemmy_domain: String,
}

#[derive(Clone)]
struct Env {
    flairs_port: u16,
    lemmy_port: u16,
    docker: bool,
    db_path: String,
    lemmy_domain: String,
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    dotenv().ok();
    tracing_subscriber::fmt::init();

    let args = cli::Args::parse();
    match &args.command {
        Some(Commands::Serve) => {
            let env = load_env();
            println!("The flair server is now running with {} on port {}, polling the {} Lemmy instance on port {}!", if env.docker {"Docker"} else {"Cargo"}, env.flairs_port, env.lemmy_domain, env.lemmy_port);

            // Database setup
            let db_config = deadpool_sqlite::Config::new(env.db_path);
            let pool = db_config.create_pool(Runtime::Tokio1)?;
            init_db(&pool).await?;

            let app_state = AppState {
                pool,
                lemmy_port: env.lemmy_port,
                docker: env.docker,
                lemmy_domain: env.lemmy_domain,
            };

            let app = Router::new()
                .route("/", routing::get(router::render_index))
                .route("/api/v1/user", routing::get(router::get_user_flair_api))
                .route("/api/v1/user", routing::put(router::put_user_flair_api))
                .route("/api/v1/user", routing::delete(router::delete_user_api))
                .route(
                    "/api/v1/community",
                    routing::get(router::get_community_flairs_api),
                )
                .route(
                    "/api/v1/community",
                    routing::put(router::put_community_flairs_api),
                )
                .route(
                    "/api/v1/community",
                    routing::delete(router::delete_community_flairs_api),
                )
                .route(
                    "/api/v1/setup",
                    routing::get(router::get_community_list_api),
                )
                .with_state(app_state);

            let addr = SocketAddr::from(([0, 0, 0, 0], env.flairs_port));
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

/// Loads the following environment variables:
/// - `FLAIRS_PORT int`
/// - `LEMMY_PORT int`
/// - `DOCKER bool`
/// - `FLAIR_DB_URL string`
fn load_env() -> Env {
    // Retrieve port where flair will run - defaults to 6969
    let mut flairs_port_env = env::var("FLAIRS_PORT").unwrap_or(String::from("6969"));
    if flairs_port_env.starts_with(":") {
        eprintln!("Please remove the ':' on your FLAIRS_PORT environment variable");
        flairs_port_env = flairs_port_env.trim_start_matches(":").to_string();
    }
    let flairs_port: u16 = flairs_port_env.parse().unwrap_or(6969);

    // Retrieve port where Lemmy is running - defaults to 8536
    let mut lemmy_port_env = env::var("LEMMY_PORT").unwrap_or(String::from("8536"));
    if lemmy_port_env.starts_with(":") {
        eprintln!("Please remove the ':' on your LEMMY_PORT environment variable");
        lemmy_port_env = lemmy_port_env.trim_start_matches(":").to_string();
    }
    let lemmy_port: u16 = lemmy_port_env.parse().unwrap_or(8536);

    // Check if Flair is running in a Docker container (true, default) or on bare metal with Cargo (false)
    let docker_env = env::var("DOCKER").unwrap_or(String::from("true"));
    let docker: bool = docker_env.parse().unwrap_or(false);

    // Retrieve the path where the sqlite DB should be saved
    let db_path = env::var("FLAIR_DB_URL").unwrap_or(String::from("./database/flairs.db"));

    // Retrieve the domain of the current lemmy instance
    let lemmy_domain =
        env::var("LEMMY_DOMAIN").expect("The LEMMY_DOMAIN environment variable must be set");

    return Env {
        flairs_port,
        lemmy_port,
        docker,
        db_path,
        lemmy_domain,
    };
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
