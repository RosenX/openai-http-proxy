-- Add down migration script here

ALTER TABLE feed_item ALTER COLUMN cover TYPE VARCHAR(200);