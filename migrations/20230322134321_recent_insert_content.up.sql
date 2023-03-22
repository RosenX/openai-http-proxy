-- Add up migration script here

CREATE TABLE recent_insert_content (
    md5 VARCHAR(50) NOT NULL UNIQUE
);
