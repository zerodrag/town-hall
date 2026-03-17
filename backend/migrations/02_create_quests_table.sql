CREATE TABLE IF NOT EXISTS quests(
    quest_id    INT          PRIMARY KEY GENERATED ALWAYS AS IDENTITY,
    poster_id   INT,
    title       VARCHAR(100) NOT NULL,
    description TEXT         NOT NULL,
    created_at  TIMESTAMPTZ  NOT NULL DEFAULT CURRENT_TIMESTAMP,

    CONSTRAINT fk_user
        FOREIGN KEY (poster_id)
        REFERENCES users(user_id)
        ON DELETE SET NULL
)
