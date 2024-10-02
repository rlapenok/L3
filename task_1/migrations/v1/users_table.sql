CREATE TABLE IF NOT EXISTS users (
    login TEXT NOT NULL UNIQUE,
    user_uid UUID NOT NULL UNIQUE,
    hashed_password TEXT NOT NULL   
);