-- Add up migration script here
--command: sqlx migrate run
CREATE TABLE IF NOT EXISTS questions (
    id CHAR(36) PRIMARY KEY NOT NULL,
    title VARCHAR(255) NOT NULL UNIQUE,
    content TEXT NOT NULL
);