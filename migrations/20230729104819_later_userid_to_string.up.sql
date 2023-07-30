-- Add up migration script here


ALTER TABLE feed ALTER COLUMN user_id TYPE VARCHAR(50);
ALTER TABLE feed_item ALTER COLUMN user_id TYPE VARCHAR(50);
ALTER TABLE feed_group ALTER COLUMN user_id TYPE VARCHAR(50);
ALTER TABLE feed_update_record ALTER COLUMN user_id TYPE VARCHAR(50);