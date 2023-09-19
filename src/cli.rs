use anyhow::anyhow;
use clap::{Parser, Subcommand};
use deadpool_sqlite::Pool;

/// Flairs augments the Lemmy Fediverse software by adding user flairs like Reddit.
/// Set RUST_LOG = debug to see log messages.
#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None, arg_required_else_help = true)]
pub(crate) struct Args {
    #[command(subcommand)]
    pub(crate) command: Option<Commands>,
}

#[derive(Subcommand, Debug)]
pub(crate) enum Commands {
    /// Start the flair's webserver
    Serve,
}

/// init_db initalizes the required database structure for Flairs to operate.
pub(crate) async fn init_db(pool: &Pool) -> anyhow::Result<()> {
    let conn = pool.get().await?;
    if let Err(e) = conn
        .interact(|conn| {

            let mut flr_stmt = conn
                .prepare(
                    r"
            CREATE TABLE IF NOT EXISTS flairs (
                id INTEGER PRIMARY KEY AUTOINCREMENT,
                name TEXT NOT NULL,
                display_name TEXT NOT NULL,
                path TEXT,
                assigned_on TEXT NOT NULL, -- Representing DateTime<Utc> as TEXT in SQLite ISO format
                community_actor_id TEXT NOT NULL,
                mod_only BOOLEAN NOT NULL
            );",
                )
                .unwrap();
            flr_stmt.execute([]).unwrap();

            let mut user_flr_stmt = conn
            .prepare(
            r"
                CREATE TABLE IF NOT EXISTS user_flairs (
                    id INTEGER PRIMARY KEY AUTOINCREMENT,
                    user_actor_id TEXT NOT NULL,
                    flair_id INTEGER NOT NULL,
                    assigned_on TEXT NOT NULL,
                    FOREIGN KEY (flair_id) REFERENCES flairs(id)
                );",
            ).unwrap();
            user_flr_stmt.execute([]).unwrap();
        })
        .await


    {
        return Err(anyhow!("unable to initalize required table {:?}", e));
    }

    Ok(())
}
