-- Add down migration script here


ALTER TABLE feed  ALTER COLUMN last_sync_device TYPE VARCHAR(100);
ALTER TABLE feed_item  ALTER COLUMN last_sync_device TYPE VARCHAR(100);
ALTER TABLE feed_group  ALTER COLUMN last_sync_device TYPE VARCHAR(100);
ALTER TABLE feed_update_record  ALTER COLUMN last_sync_device TYPE VARCHAR(100);
