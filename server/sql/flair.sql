-- Create FlairDirectory table if it doesn't exist
CREATE TABLE IF NOT EXISTS flair_directory (
    ID serial PRIMARY KEY,
    special BOOL NOT NULL,
    ref_id VARCHAR(255),
    pos INT,
    flair VARCHAR(255) NOT NULL,
    path VARCHAR(255)
);

-- Create Flairs table if it doesn't exist
CREATE TABLE IF NOT EXISTS flairs (
    ID serial PRIMARY KEY,
    name VARCHAR(255) NOT NULL,
    assigned_on TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
    flair INT REFERENCES flair_directory(ID)
);
