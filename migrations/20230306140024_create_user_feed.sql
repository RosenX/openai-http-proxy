-- Add migration script here

CREATE TABLE IF NOT EXISTS user_feed(
    user_id INT NOT NULL,
    url VARCHAR(200) NOT NULL,
    name VARCHAR(30) NOT NULL,
    icon VARCHAR(200),
    created_time TIMESTAMP NOT NULL,
    PRIMARY KEY (user_id, url),
    FOREIGN KEY (user_id) REFERENCES user_profile(user_id)
);