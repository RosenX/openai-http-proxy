-- Add up migration script here

CREATE OR REPLACE FUNCTION content_insert_trigger() RETURNS TRIGGER AS $$
BEGIN
    INSERT INTO user_content (content_id, user_id)
    SELECT DISTINCT NEW.id, user_id FROM user_feed WHERE feed_id = NEW.feed_id;
    RETURN NULL;
END;
$$ LANGUAGE plpgsql;

CREATE TRIGGER content_trigger
    AFTER INSERT ON content
    FOR EACH ROW EXECUTE PROCEDURE content_insert_trigger();
