-- Add down migration script here

ALTER TABLE feed ALTER COLUMN logo TYPE VARCHAR(500);
ALTER TABLE feed ALTER COLUMN custom_logo TYPE VARCHAR(500);