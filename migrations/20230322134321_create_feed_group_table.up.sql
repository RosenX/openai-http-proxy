-- Add up migration script here

CREATE TABLE feed_group (
    user_id INTEGER NOT NULL,
    group_id INTEGER NOT NULL,
    name VARCHAR(255) NOT NULL,
    description VARCHAR(255) NOT NULL,

    PRIMARY KEY (user_id, group_id)
);
