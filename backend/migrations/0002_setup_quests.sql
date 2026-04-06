CREATE TABLE quests (
    quest_id BIGINT GENERATED ALWAYS AS IDENTITY PRIMARY KEY,
    poster_id BIGINT NOT NULL REFERENCES users (user_id),

    title TEXT NOT NULL,
    CONSTRAINT title_length_is_valid CHECK (
        char_length(title) BETWEEN 1 AND 100
    ),

    description TEXT,

    status TEXT NOT NULL DEFAULT 'draft',
    CONSTRAINT status_is_valid CHECK (
        status IN ('draft', 'ongoing', 'solved')
    ),

    CONSTRAINT description_required_for_non_draft CHECK (
        status = 'draft' OR description IS NOT NULL
    ),

    created_at TIMESTAMPTZ NOT NULL DEFAULT current_timestamp
);
