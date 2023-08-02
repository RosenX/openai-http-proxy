-- Add up migration script here


ALTER TABLE feed_item ADD COLUMN is_achieved BOOLEAN NOT NULL DEFAULT FALSE;