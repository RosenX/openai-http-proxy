-- Add down migration script here

ALTER TABLE feed_item ALTER COLUMN link SET NOT NULL;
