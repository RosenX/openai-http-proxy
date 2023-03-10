-- Add migration script here

CREATE TABLE IF NOT EXISTS feed_post (
    id INT AUTO_INCREMENT PRIMARY KEY NOT NULL COMMENT '自增ID',
    feed_id INT NOT NULL COMMENT 'Feed ID',
    title VARCHAR(50) NOT NULL COMMENT '标题',
    publish_time TIMESTAMP NOT NULL COMMENT '发表时间',
    authors VARCHAR(100) COMMENT '作者',
    link VARCHAR(200) COMMENT '原文链接',
    content MEDIUMTEXT NOT NULL COMMENT '文本内容',
    summary VARCHAR(300) COMMENT '官方Summary',
    summary_algo VARCHAR(300) COMMENT '算法Summary',
    tags_algo VARCHAR(100) COMMENT '算法标签',
    category_algo VARCHAR(100) COMMENT '算法分类',
    FOREIGN KEY (feed_id) REFERENCES feed_profile(id)
);