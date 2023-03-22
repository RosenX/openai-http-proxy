-- Add up migration script here

-- CREATE TYPE read_stage AS ENUM('explore', 'focus', 'seen', 'archive');

CREATE TABLE user_content (
    id SERIAL,
    content_id INTEGER NOT NULL,
    user_id INTEGER NOT NULL,
    stage SMALLINT NOT NULL,
    tags VARCHAR(500),
    category VARCHAR(50),
    notes TEXT,

    PRIMARY KEY (content_id, user_id)
);

COMMENT ON COLUMN user_content.stage IS '0-Explore,1-Focus,2-Seen,3-Archive';

-- CREATE INDEX user_content_id_idx ON user_content (id);
-- CREATE INDEX user_content_user_id_idx ON user_content (user_id);
