-- Add up migration script here

ALTER TABLE feed_update_record ADD COLUMN failed_count INT DEFAULT 0;

