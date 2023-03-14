-- Add migration script here

ALTER TABLE user_custom_post ADD stage INT NOT NULL DEFAULT 1 COMMENT "阶段：1-Explore;2-Focus;3-Seen;4-Archive";