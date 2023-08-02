-- Add up migration script here


ALTER TABLE feed_item ADD COLUMN is_archived BOOLEAN NOT NULL DEFAULT FALSE;