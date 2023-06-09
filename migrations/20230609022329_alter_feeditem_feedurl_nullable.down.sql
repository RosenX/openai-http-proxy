-- Add down migration script here


ALTER TABLE feed_item ALTER COLUMN feed_url TYPE VARCHAR(200) NOT NULL;