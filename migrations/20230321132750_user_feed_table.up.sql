-- Add up migration script here

CREATE TABLE user_feed (
    id SERIAL,
    user_id INTEGER NOT NULL,
    feed_id INTEGER NOT NULL,
    name VARCHAR(50),
    logo VARCHAR(200),
    description VARCHAR(300),
    created_time TIMESTAMP(0) WITH TIME ZONE NOT NULL,
    folder VARCHAR(50),
    tags VARCHAR(500),

    PRIMARY KEY (user_id, feed_id)
);

-- CREATE INDEX user_feed_user_id_idx ON user_feed (user_id);
