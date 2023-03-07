-- Add migration script here

ALTER TABLE user_profile MODIFY created_time TIMESTAMP;
ALTER TABLE user_profile MODIFY pro_end_time TIMESTAMP;