-- Add migration script here

CREATE TABLE IF NOT EXISTS user_custom_post (
    post_id INT NOT NULL COMMENT 'Feed ID',
    user_id INT NOT NULL COMMENT 'User ID',
    tags VARCHAR(100) COMMENT '标签',
    category VARCHAR(100) COMMENT '分类',
    notes TEXT COMMENT '用户笔记',
    PRIMARY KEY (user_id, post_id),
    FOREIGN KEY (user_id) REFERENCES user_profile(id),
    FOREIGN KEY (post_id) REFERENCES feed_post(id)
);