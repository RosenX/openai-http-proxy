-- Add up migration script here

CREATE TYPE FeedType AS ENUM('Rss', 'Atom', 'Unknown');

CREATE TABLE feed (
    user_id INTEGER NOT NULL,
    feed_id INTEGER NOT NULL,
    url VARCHAR(200) NOT NULL UNIQUE,
    name VARCHAR(100) NOT NULL,
    custom_name VARCHAR(100),
    logo VARCHAR(200),
    custom_logo VARCHAR(200),
    description VARCHAR(500),
    custom_description VARCHAR(500),
    group_id INTEGER,
    tags VARCHAR(500),
    create_time TIMESTAMP(0) WITH TIME ZONE NOT NULL,
    type FeedType,
    update_time TIMESTAMP(0) WITH TIME ZONE NOT NULL,
    PRIMARY KEY (user_id, feed_id)
);
