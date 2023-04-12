-- Add up migration script here

CREATE TABLE feed_update_record (
    user_id INTEGER NOT NULL,
    feed_id INTEGER NOT NULL,
    last_update TIMESTAMP(0) WITH TIME ZONE NOT NULL,
    last_content_hash VARCHAR(200),
    last_item_publish_time TIMESTAMP(0) WITH TIME ZONE NOT NULL,

    PRIMARY KEY (user_id, feed_id)
);
