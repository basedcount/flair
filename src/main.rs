use axum::{routing::get, Router};
use bb8::Pool;
use bb8_postgres::PostgresConnectionManager;
use dotenv::dotenv;
use std::net::SocketAddr;
use std::env;
use tokio_postgres::NoTls;

mod router;

type ConnectionPool = Pool<PostgresConnectionManager<NoTls>>;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    dotenv().ok();
    tracing_subscriber::fmt::init();

    let flairs_port = env::var("FLAIRS_PORT").unwrap_or(String::from("6969"));
    if !flairs_port.starts_with(":") {
        eprintln!("Please remove the ':' on your FLAIRS_PORT environment variable");
    }

    let port: u16 = flairs_port.parse().unwrap_or(6969);

    // Database setup
    let db_conn_string =
        env::var("LEMMY_DATABASE_URL").expect("LEMMY_DATABASE_URL ENVIRONMENT VARIABLE is not set");
    let manager =
        PostgresConnectionManager::new_from_stringlike(&db_conn_string, NoTls)?;

    let pool = Pool::builder().build(manager).await.unwrap();

  
    let app = Router::new()
        .route("/", get(router::get_user_flair))
        .with_state(pool);

    let addr = SocketAddr::from(([127, 0, 0, 1], port));
    tracing::debug!("listening on {}", addr);
    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();

    Ok(())
}
