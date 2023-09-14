-- Add down migration script here

ALTER TABLE user_activity DROP COLUMN system;
ALTER TABLE user_activity DROP COLUMN system_version;