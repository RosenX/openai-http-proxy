-- Add up migration script here

CREATE TABLE feed_item (
    user_id INTEGER NOT NULL,
    item_id INTEGER NOT NULL,
    is_focus BOOLEAN NOT NULL,
    is_seen BOOLEAN NOT NULL,
    title VARCHAR(200) NOT NULL,
    cover VARCHAR(200),
    link VARCHAR(200) NOT NULL,
    publish_time TIMESTAMP(0) WITH TIME ZONE NOT NULL,
    authors VARCHAR(200),
    tags VARCHAR(500),
    category VARCHAR(200),
    description VARCHAR(500),
    summary_algo VARCHAR(200),
    content VARCHAR(5000),
    content_have_parsed BOOLEAN NOT NULL,
    create_time TIMESTAMP(0) WITH TIME ZONE NOT NULL,
    md5_string VARCHAR(200) NOT NULL,
    feed_id INTEGER NOT NULL,

    update_time TIMESTAMP(0) WITH TIME ZONE NOT NULL,

    PRIMARY KEY (user_id, item_id)
);
