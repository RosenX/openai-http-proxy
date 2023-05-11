-- Add up migration script here

CREATE TABLE feed_group (
    user_id INTEGER NOT NULL,
    name VARCHAR(255) NOT NULL,
    description VARCHAR(255),

    update_time TIMESTAMP(0) WITH TIME ZONE NOT NULL,
    sync_time TIMESTAMP(0) WITH TIME ZONE NOT NULL,
    sync_devices INTEGER[] NOT NULL,

    PRIMARY KEY (user_id, name)
);
