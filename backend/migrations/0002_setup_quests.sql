CREATE TYPE quest_status AS ENUM ('draft', 'ongoing', 'solved');

CREATE TABLE quests (
    quest_id BIGINT GENERATED ALWAYS AS IDENTITY PRIMARY KEY,

    poster_id BIGINT NOT NULL REFERENCES users (user_id),

    title TEXT NOT NULL,

    summary TEXT NOT NULL DEFAULT '',
    details TEXT NOT NULL DEFAULT '',

    techs TEXT[] NOT NULL DEFAULT '{}',

    status quest_status NOT NULL DEFAULT 'draft',

    created_at TIMESTAMPTZ NOT NULL DEFAULT current_timestamp,

    CONSTRAINT title_length_is_valid CHECK (
        char_length (title) BETWEEN 1 AND 100
    )
);
