CREATE OR REPLACE FUNCTION notify_changes() RETURNS TRIGGER AS $$
BEGIN
    IF (TG_OP = 'INSERT') THEN
        PERFORM pg_notify(
            'inserts',  
            json_build_object(
                'operation', TG_OP,
                'id', NEW.id,               
                'description', NEW.description,
                'created_at',NEW.created_at,
                'completed_at',NEW.completed_at,
                'trace_id', NEW.trace_id,
                'span_id', NEW.span_id        
            )::text
        );
    ELSIF (TG_OP = 'UPDATE') THEN
        PERFORM pg_notify(
            'updates',  
            json_build_object(
                'operation', TG_OP,      
                'id', NEW.id,               
                'description', NEW.description,
                'created_at',NEW.created_at,
                'completed_at',NEW.completed_at, 
                'trace_id', NEW.trace_id,
                'span_id', NEW.span_id  
            )::text
        );
    END IF;
    RETURN NULL;
END;
$$ LANGUAGE plpgsql;
