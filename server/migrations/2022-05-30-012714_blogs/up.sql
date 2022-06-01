-- Your SQL goes here
-- 程序运行的时候执行
CREATE TABLE blogs(
    id CHARACTER(36) NOT NULL PRIMARY KEY,
    user_id CHARACTER(36) NOT NULL,
    title Text NOT NULL,
    content Text NOT NULL,
    tag Text,
    image BLOB,
    image_url VARCHAR(200),
    likes INTEGER,
    page_view_num INTEGER,
    is_deleted BOOLEAN NOT NULL DEFAULT 'f',
    created_at DATETIME DEFAULT CURRENT_TIMESTAMP NOT NULL,
    updated_at DATETIME DEFAULT CURRENT_TIMESTAMP NOT NULL
)