CREATE EXTENSION IF NOT EXISTS pg_trgm;

CREATE TYPE quest_status AS ENUM ('draft', 'ongoing', 'solved');

CREATE TABLE quests (
    quest_id BIGINT GENERATED ALWAYS AS IDENTITY PRIMARY KEY,

    poster_id BIGINT NOT NULL REFERENCES users (user_id),

    title TEXT NOT NULL,

    summary TEXT NOT NULL DEFAULT '',
    details TEXT NOT NULL DEFAULT '',

    techs TEXT[] NOT NULL DEFAULT '{}',

    status quest_status NOT NULL DEFAULT 'draft',

    created_at TIMESTAMPTZ NOT NULL DEFAULT current_timestamp
);

CREATE INDEX quests_title_trgm_idx ON quests USING GIN (title gin_trgm_ops);
CREATE INDEX quests_summary_trgm_idx ON quests USING GIN (summary gin_trgm_ops);
CREATE INDEX quests_techs_gin_idx ON quests USING GIN (techs);
