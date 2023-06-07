-- Add down migration script here

ALTER TABLE feed_item ALTER COLUMN link TYPE VARCHAR(200);