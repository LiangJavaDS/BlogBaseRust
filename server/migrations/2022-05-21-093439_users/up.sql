-- Your SQL goes here
CREATE TABLE IF NOT EXISTS users(
    id CHARACTER(36) NOT NULL PRIMARY KEY,
    username VARCHAR(50),
    password VARCHAR(50),
    email VARCHAR(60),
    phone VARCHAR(60),
    avatar BLOB,
    avatar_url VARCHAR(200),
    slogan VARCHAR(60),
    is_deleted INTEGER,
    created_at DATETIME DEFAULT CURRENT_TIMESTAMP NOT NULL,
    updated_at DATETIME DEFAULT CURRENT_TIMESTAMP NOT NULL
)