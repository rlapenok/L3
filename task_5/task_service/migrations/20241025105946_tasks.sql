CREATE TABLE IF NOT EXISTS tasks (
    id UUID NOT NULL UNIQUE,   
    description TEXT NOT NULL,
    created_at  TIMESTAMPTZ NOT NULL,
    completed_at  TIMESTAMPTZ,
    completed BOOLEAN DEFAULT FALSE NOT NULL,
    trace_id TEXT NOT NULL,
    span_id TEXT NOT NULL
);