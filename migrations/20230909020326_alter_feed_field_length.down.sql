-- Add down migration script here

ALTER TABLE feed ALTER COLUMN url TYPE VARCHAR(200);
ALTER TABLE feed ALTER COLUMN logo TYPE VARCHAR(200);
ALTER TABLE feed ALTER COLUMN custom_logo TYPE VARCHAR(200);
ALTER TABLE feed ALTER COLUMN last_sync_device TYPE VARCHAR(200);
