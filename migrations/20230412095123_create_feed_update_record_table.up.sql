-- Add up migration script here

CREATE TABLE feed_update_record (
    user_id INTEGER NOT NULL,
    feed_url VARCHAR(200) NOT NULL,
    last_update TIMESTAMP(0) WITH TIME ZONE NOT NULL,
    last_content_hash VARCHAR(200),
    last_item_publish_time TIMESTAMP(0) WITH TIME ZONE NOT NULL,

    update_time TIMESTAMP(0) WITH TIME ZONE NOT NULL,
    sync_time TIMESTAMP(0) WITH TIME ZONE NOT NULL,
    sync_devices INTEGER[] NOT NULL,

    PRIMARY KEY (user_id, feed_url)
);
