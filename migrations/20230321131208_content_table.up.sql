-- Add up migration script here

CREATE TABLE content (
    id SERIAL PRIMARY KEY,
    feed_id INTEGER,
    title VARCHAR(50) NOT NULL UNIQUE,
    publish_time TIMESTAMP(0) WITH TIME ZONE,
    create_time TIMESTAMP(0) WITH TIME ZONE NOT NULL,
    authors VARCHAR(100),
    link VARCHAR(200),
    content TEXT,
    cover VARCHAR(200),
    summary TEXT,
    summary_algo TEXT,
    tags_algo VARCHAR(500),
    category_algo VARCHAR(50),
    md5 CHAR(50) NOT NULL UNIQUE
);

COMMENT ON COLUMN content.md5 IS 'title+content md5 32位';
