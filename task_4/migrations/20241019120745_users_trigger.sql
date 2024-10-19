-- Add migration script here
CREATE TRIGGER users_changes_trigger
AFTER INSERT OR UPDATE OR DELETE ON users
FOR EACH ROW EXECUTE FUNCTION notify_changes();