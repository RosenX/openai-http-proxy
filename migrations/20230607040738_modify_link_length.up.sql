-- Add up migration script here

-- alter link in feeditem to varchar(500)
ALTER TABLE feed_item ALTER COLUMN link TYPE VARCHAR(500);