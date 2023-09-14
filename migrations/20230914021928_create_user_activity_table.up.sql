-- Add up migration script here

CREATE TABLE user_activity (
    device_id VARCHAR(200) NOT NULL,
    date Date NOT NULL,
    device_type VARCHAR(100) NOT NULL,
    user_id VARCHAR(100),
    use_times INTEGER NOT NULL DEFAULT 0,
    feed_num INTEGER NOT NULL,
    keyword_num INTEGER NOT NULL,
    PRIMARY KEY (device_id, date)
);