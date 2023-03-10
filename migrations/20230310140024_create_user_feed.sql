-- Add migration script here

CREATE TABLE IF NOT EXISTS user_custom_feed(
    user_id INT NOT NULL COMMENT 'User ID',
    feed_id INT NOT NULL COMMENT 'Feed ID',
    name VARCHAR(30) COMMENT '用户自定义Feed Name',
    description VARCHAR(300) COMMENT '用户自定义Feed Description',
    logo VARCHAR(200) COMMENT '用户自定义Feed Logo URL',
    icon VARCHAR(200) COMMENT '用户自定义Feed Icon URL',
    created_time TIMESTAMP NOT NULL COMMENT 'Feed添加时间',
    PRIMARY KEY (user_id, feed_id),
    FOREIGN KEY (user_id) REFERENCES user_profile(id),
    FOREIGN KEY (feed_id) REFERENCES feed_profile(id)
);