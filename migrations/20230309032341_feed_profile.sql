-- Add migration script here

CREATE TABLE IF NOT EXISTS feed_profile(
    id INT AUTO_INCREMENT PRIMARY KEY NOT NULL COMMENT 'Feed ID',
    url VARCHAR(200) NOT NULL COMMENT 'Feed URL',
    name VARCHAR(30) NOT NULL COMMENT 'Feed Name',
    logo VARCHAR(200) NOT NULL COMMENT 'LOGO URL',
    description VARCHAR(300) COMMENT 'Feed Description',
    homepage VARCHAR(200) COMMENT 'Feed Homepage',
    icon VARCHAR(200) COMMENT 'Feed Icon, Smaller than logo',
    category VARCHAR(50) COMMENT '官方分类或者系统生成分类，逗号分割',
    tags VARCHAR(200) COMMENT '官方标签或者系统生成标签，逗号分割'
);