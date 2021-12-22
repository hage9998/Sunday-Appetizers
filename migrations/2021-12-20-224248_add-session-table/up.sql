CREATE TABLE IF NOT EXISTS pate.sessions(
    login uuid PRIMARY KEY,
    token TEXT NOT NULL,
    created TIMESTAMP DEFAULT NOW()
);
