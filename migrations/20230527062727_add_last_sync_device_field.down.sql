-- Add down migration script here

ALTER TABLE feed DROP COLUMN last_sync_device;
ALTER TABLE feed_item DROP COLUMN last_sync_device;
ALTER TABLE feed_group DROP COLUMN last_sync_device;
ALTER TABLE feed_update_record DROP COLUMN last_sync_device;


-- change sync_device to not nullable
ALTER TABLE feed ALTER COLUMN sync_devices SET NOT NULL;
ALTER TABLE feed_item ALTER COLUMN sync_devices SET NOT NULL;
ALTER TABLE feed_group ALTER COLUMN sync_devices SET NOT NULL;
ALTER TABLE feed_update_record ALTER COLUMN sync_devices SET NOT NULL;
