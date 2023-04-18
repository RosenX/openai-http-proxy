-- Add up migration script here

CREATE TABLE user_device (
    id SERIAL,
    user_id INTEGER NOT NULL,
    device_name VARCHAR(200) NOT NULL
);
