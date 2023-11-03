-- Create UserFlairs table if it doesn't exist
CREATE TABLE IF NOT EXISTS user_flairs (
    user_actor_id TEXT NOT NULL,
    flair_name TEXT NOT NULL,
    flair_community_actor_id TEXT NOT NULL,
    assigned_on TEXT NOT NULL,
    FOREIGN KEY (flair_name, flair_community_actor_id) REFERENCES flairs(name, community_actor_id) ON DELETE CASCADE,
    PRIMARY KEY (user_actor_id, flair_community_actor_id)
);

-- Create Flairs table if it doesn't exist
CREATE TABLE IF NOT EXISTS flairs (
    name TEXT NOT NULL,
    display_name TEXT NOT NULL,
    path TEXT,
    community_actor_id TEXT NOT NULL,
    mod_only BOOLEAN NOT NULL,
    PRIMARY KEY(name, community_actor_id)
);
