-- Add up migration script here

ALTER TABLE feed_update_record ALTER COLUMN last_item_publish_time DROP NOT NULL;