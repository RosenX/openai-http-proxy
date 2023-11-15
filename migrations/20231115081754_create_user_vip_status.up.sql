-- Add up migration script here
-- Add up migration script here

CREATE TABLE vip_status (
    user_id VARCHAR(100) PRIMARY KEY,
    is_forever BOOLEAN NOT NULL DEFAULT FALSE,
    pro_end_time TIMESTAMP(0) WITH TIME ZONE NOT NULL
);
