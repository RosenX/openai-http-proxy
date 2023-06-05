-- Add up migration script here


ALTER TABLE feed_item ADD COLUMN is_marked BOOLEAN DEFAULT FALSE;
