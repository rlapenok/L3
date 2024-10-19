-- Add migration script here
CREATE TRIGGER products_changes_trigger
AFTER INSERT OR UPDATE OR DELETE ON products
FOR EACH ROW EXECUTE FUNCTION notify_changes();