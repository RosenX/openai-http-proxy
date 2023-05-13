-- Add up migration script here

CREATE TYPE feed_type AS ENUM('rss', 'atom', 'unknown');

CREATE TABLE feed (
    user_id INTEGER NOT NULL,
    url VARCHAR(200) NOT NULL,
    name VARCHAR(100) NOT NULL,
    custom_name VARCHAR(100),
    logo VARCHAR(200),
    custom_logo VARCHAR(200),
    description TEXT,
    custom_description TEXT,
    group_name VARCHAR(100),
    tags VARCHAR(100)[],
    create_time TIMESTAMP(0) WITH TIME ZONE NOT NULL,
    feed_type feed_type,
    update_time TIMESTAMP(0) WITH TIME ZONE NOT NULL,
    sync_time TIMESTAMP(0) WITH TIME ZONE NOT NULL,
    sync_devices INTEGER[] NOT NULL,
    is_deleted BOOLEAN NOT NULL,
    PRIMARY KEY (user_id, url)
);
