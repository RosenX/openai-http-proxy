-- Add migration script here

CREATE TABLE IF NOT EXISTS user_profile(
    user_id INT AUTO_INCREMENT PRIMARY KEY NOT NULL,
    username VARCHAR(20) NOT NULL,
    email VARCHAR(50) UNIQUE NOT NULL,
    password VARCHAR(200) NOT NULL,
    pro_level INT NOT NULL DEFAULT 0,
    created_time DATETIME NOT NULL,
    pro_end_time DATETIME NOT NULL
);