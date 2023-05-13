-- Add up migration script here

CREATE TABLE feed_item (
    user_id INTEGER NOT NULL,
    is_focus BOOLEAN NOT NULL,
    is_seen BOOLEAN NOT NULL,
    title VARCHAR(200) NOT NULL,
    cover VARCHAR(200),
    link VARCHAR(200) NOT NULL,
    publish_time TIMESTAMP(0) WITH TIME ZONE NOT NULL,
    authors VARCHAR(200),
    tags VARCHAR(100)[],
    category VARCHAR(200),
    description TEXT,
    summary_algo TEXT,
    create_time TIMESTAMP(0) WITH TIME ZONE NOT NULL,
    md5_string VARCHAR(200) NOT NULL,
    feed_url VARCHAR(200) NOT NULL,

    update_time TIMESTAMP(0) WITH TIME ZONE NOT NULL,
    sync_time TIMESTAMP(0) WITH TIME ZONE NOT NULL,
    sync_devices INTEGER[] NOT NULL,
    is_deleted BOOLEAN NOT NULL,

    PRIMARY KEY (user_id, md5_string)
);
