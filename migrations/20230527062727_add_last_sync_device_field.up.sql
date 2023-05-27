-- Add up migration script here

ALTER TABLE feed ADD COLUMN last_sync_device VARCHAR(100) NOT NULL DEFAULT '';
ALTER TABLE feed_item ADD COLUMN last_sync_device VARCHAR(100) NOT NULL DEFAULT '';
ALTER TABLE feed_group ADD COLUMN last_sync_device VARCHAR(100) NOT NULL DEFAULT '';
ALTER TABLE feed_update_record ADD COLUMN last_sync_device VARCHAR(100) NOT NULL DEFAULT '';

-- change sync_device to nullable
ALTER TABLE feed ALTER COLUMN sync_devices DROP NOT NULL;
ALTER TABLE feed_item ALTER COLUMN sync_devices DROP NOT NULL;
ALTER TABLE feed_group ALTER COLUMN sync_devices DROP NOT NULL;
ALTER TABLE feed_update_record ALTER COLUMN sync_devices DROP NOT NULL;
