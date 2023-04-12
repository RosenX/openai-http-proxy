-- Add up migration script here

CREATE TYPE ProLevel AS ENUM('Normal', 'PRO', 'SPro');

CREATE TABLE user_information (
    id SERIAL PRIMARY KEY,
    username VARCHAR(50) NOT NULL,
    email VARCHAR(50) UNIQUE NOT NULL,
    password VARCHAR(200) NOT NULL,
    pro_level ProLevel DEFAULT 'Normal' NOT NULL,
    created_time TIMESTAMP(0) WITH TIME ZONE NOT NULL,
    pro_end_time TIMESTAMP(0) WITH TIME ZONE NOT NULL
);
