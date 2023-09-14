-- Add up migration script here

ALTER TABLE user_activity ADD COLUMN system VARCHAR(30) NOT NULL ;
ALTER TABLE user_activity ADD COLUMN system_version VARCHAR(30) NOT NULL ;
