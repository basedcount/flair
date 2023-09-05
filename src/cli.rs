use anyhow::anyhow;
use clap::{Parser, Subcommand};
use deadpool_sqlite::Pool;

/// Flairs augments the Lemmy Fediverse software by adding user flairs like Reddit.
#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
pub(crate) struct Args {
    #[command(subcommand)]
    pub(crate) command: Option<Commands>,
}

#[derive(Subcommand, Debug)]
pub(crate) enum Commands {
    Serve,
}

pub(crate) async fn init_db(pool: &Pool) -> anyhow::Result<()> {
    let conn = pool.get().await?;
    if let Err(e) = conn
        .interact(|conn| {
            let mut flr_dir_stmt = conn
                .prepare(
                    r"
        CREATE TABLE IF NOT EXISTS flair_directory (
            ID serial PRIMARY KEY,
            special BOOL NOT NULL,
            ref_id VARCHAR(255),
            pos INT,
            flair VARCHAR(255) NOT NULL,
            path VARCHAR(255)
        );
        ",
                )
                .unwrap();
            flr_dir_stmt.execute([]).unwrap();

            let mut flr_stmt = conn.prepare(r"
            CREATE TABLE IF NOT EXISTS flairs (
                ID serial PRIMARY KEY,
                name VARCHAR(255) NOT NULL,
                assigned_on TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
                flair INT REFERENCES flair_directory(ID)
            );").unwrap();
            flr_stmt.execute([]).unwrap()
        })
        .await
    {
        return Err(anyhow!("unable to initalize required table {:?}", e));
    }

    Ok(())
}
