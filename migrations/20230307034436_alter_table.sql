-- Add migration script here
ALTER TABLE user_profile MODIFY created_time TIMESTAMP NOT NULL;
ALTER TABLE user_profile MODIFY pro_end_time TIMESTAMP NOT NULL;

ALTER TABLE user_feed ADD COLUMN created_time TIMESTAMP NOT NULL;