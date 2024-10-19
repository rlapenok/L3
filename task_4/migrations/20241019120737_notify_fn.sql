CREATE OR REPLACE FUNCTION notify_changes() RETURNS TRIGGER AS $$
BEGIN
    IF (TG_OP = 'INSERT') THEN
        PERFORM pg_notify(
            TG_TABLE_NAME,
            json_build_object(
                'operation', TG_OP,
                'id', NEW.id,
                'data', row_to_json(NEW)
            )::text
        );
    ELSIF (TG_OP = 'UPDATE') THEN
        PERFORM pg_notify(
            TG_TABLE_NAME,
            json_build_object(
                'operation', TG_OP,
                'id', NEW.id,
                'old_data', row_to_json(OLD),
                'new_data', row_to_json(NEW)
            )::text
        );
    ELSIF (TG_OP = 'DELETE') THEN
        PERFORM pg_notify(
            TG_TABLE_NAME,
            json_build_object(
                'operation', TG_OP,
                'id', OLD.id,
                'data', row_to_json(OLD)
            )::text
        );
    END IF;
    RETURN NULL;
END;
$$ LANGUAGE plpgsql;
