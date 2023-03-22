-- Add down migration script here

DROP TRIGGER content_trigger ON content;
DROP FUNCTION content_insert_trigger();
