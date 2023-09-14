-- Add up migration script here

ALTER TABLE feed ALTER COLUMN logo TYPE VARCHAR(1000);
ALTER TABLE feed ALTER COLUMN custom_logo TYPE VARCHAR(1000);