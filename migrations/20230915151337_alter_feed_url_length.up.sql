-- Add up migration script here

ALTER TABLE feed ALTER COLUMN url TYPE VARCHAR(1000);