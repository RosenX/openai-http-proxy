-- Add up migration script here


ALTER TABLE feed ALTER COLUMN name TYPE VARCHAR(300);
ALTER TABLE feed ALTER COLUMN custom_name TYPE VARCHAR(300);
ALTER TABLE feed ALTER COLUMN group_name TYPE VARCHAR(200);