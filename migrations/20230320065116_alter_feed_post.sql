-- Add migration script here

ALTER TABLE feed_post MODIFY COLUMN summary TEXT COMMENT '文本摘要';
