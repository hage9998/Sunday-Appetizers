CREATE TABLE IF NOT EXISTS store.sessions(
    login uuid PRIMARY KEY,
    token TEXT NOT NULL,
    created TIMESTAMP DEFAULT NOW()
);
