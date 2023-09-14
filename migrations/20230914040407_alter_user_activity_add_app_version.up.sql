-- Add up migration script here

ALTER TABLE user_activity ADD COLUMN app_version VARCHAR(20) NOT NULL ;
