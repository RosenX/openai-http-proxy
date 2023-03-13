-- Add migration script here

ALTER TABLE feed_post MODIFY COLUMN content MEDIUMTEXT COMMENT '文本内容';