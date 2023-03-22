-- Add up migration script here

CREATE TABLE feed_profile (
    id SERIAL PRIMARY KEY,
    url VARCHAR(200) NOT NULL UNIQUE,
    name VARCHAR(50),
    logo VARCHAR(200),
    description VARCHAR(300),
    icon VARCHAR(300),
    category_algo VARCHAR(50),
    tags_algo VARCHAR(500)
);
