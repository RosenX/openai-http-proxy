-- Add up migration script here


-- change field length
ALTER TABLE feed  ALTER COLUMN last_sync_device TYPE VARCHAR(200);
ALTER TABLE feed_item  ALTER COLUMN last_sync_device TYPE VARCHAR(200);
ALTER TABLE feed_group  ALTER COLUMN last_sync_device TYPE VARCHAR(200);
ALTER TABLE feed_update_record  ALTER COLUMN last_sync_device TYPE VARCHAR(200);
