-- Add up migration script here

ALTER TABLE feed ADD COLUMN is_followed BOOLEAN DEFAULT FALSE;