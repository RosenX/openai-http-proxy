-- Add migration script here

CREATE TABLE IF NOT EXISTS user_feed(
    id INT AUTO_INCREMENT PRIMARY KEY NOT NULL,
    user_id INT NOT NULL,
    url VARCHAR(100) NOT NULL,
    name VARCHAR(30) NOT NULL,
    icon VARCHAR(50),
    FOREIGN KEY (user_id) REFERENCES user_profile(user_id)
);