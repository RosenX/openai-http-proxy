-- Add up migration script here


-- add is_delete field to feed_update_record
ALTER TABLE feed_update_record ADD COLUMN is_deleted BOOLEAN NOT NULL DEFAULT FALSE;
