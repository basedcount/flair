-- Create UserFlairs table if it doesn't exist
CREATE TABLE IF NOT EXISTS user_flairs (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    user_actor_id INTEGER NOT NULL,
    flair_id INTEGER NOT NULL,
    assigned_on TEXT NOT NULL,
    FOREIGN KEY (flair_id) REFERENCES flairs(id)
);

-- Create Flairs table if it doesn't exist
CREATE TABLE IF NOT EXISTS flairs (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    name TEXT NOT NULL UNIQUE,
    display_name TEXT NOT NULL,
    path TEXT,
    assigned_on TEXT NOT NULL, -- Representing DateTime<Utc> as TEXT in SQLite ISO format
    community_actor_id TEXT NOT NULL,
    mod_only BOOLEAN NOT NULL
);
