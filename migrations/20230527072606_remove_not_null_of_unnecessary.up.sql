-- Add up migration script here

ALTER TABLE feed_item ALTER COLUMN link DROP NOT NULL;
