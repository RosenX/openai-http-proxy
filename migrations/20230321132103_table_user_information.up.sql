-- Add up migration script here

-- CREATE TYPE pro_level AS ENUM('Normal', 'PRO', 'SPro');

CREATE TABLE user_information (
    id SERIAL PRIMARY KEY,
    username VARCHAR(50) NOT NULL,
    email VARCHAR(50) UNIQUE NOT NULL,
    password VARCHAR(200) NOT NULL,
    pro_level SMALLINT DEFAULT 0 NOT NULL,
    created_time TIMESTAMP(0) WITH TIME ZONE NOT NULL,
    pro_end_time TIMESTAMP(0) WITH TIME ZONE NOT NULL
);

-- CREATE INDEX user_information_email_idx ON user_information (email);

COMMENT ON COLUMN user_information.pro_level IS '0-普通,1-Pro,2-SPro';
