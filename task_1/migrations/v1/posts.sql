CREATE TABLE IF NOT EXISTS posts (
    user_uid UUID NOT NULL,
    post_uid UUID NOT NULL,   
    msg TEXT,
    likes BIGINT NOT NULL
);