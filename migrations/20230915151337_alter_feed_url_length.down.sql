-- Add down migration script here

ALTER TABLE feed ALTER COLUMN url TYPE VARCHAR(500);