CREATE TABLE "user" (
  "id" TEXT NOT NULL PRIMARY KEY,
  "name" TEXT NOT NULL,
  "email" TEXT NOT NULL UNIQUE,
  "email_verified" BOOLEAN NOT NULL,
  "image" TEXT,
  "created_at" TIMESTAMPTZ DEFAULT CURRENT_TIMESTAMP NOT NULL,
  "updated_at" TIMESTAMPTZ DEFAULT CURRENT_TIMESTAMP NOT NULL
);

CREATE TABLE "session" (
  "id" TEXT NOT NULL PRIMARY KEY,
  "expires_at" TIMESTAMPTZ NOT NULL,
  "token" TEXT NOT NULL UNIQUE,
  "created_at" TIMESTAMPTZ DEFAULT CURRENT_TIMESTAMP NOT NULL,
  "updated_at" TIMESTAMPTZ NOT NULL,
  "ip_address" TEXT,
  "user_agent" TEXT,
  "user_id" TEXT NOT NULL REFERENCES "user" ("id") ON DELETE CASCADE
);

CREATE TABLE "account" (
  "id" TEXT NOT NULL PRIMARY KEY,
  "account_id" TEXT NOT NULL,
  "provider_id" TEXT NOT NULL,
  "user_id" TEXT NOT NULL REFERENCES "user" ("id") ON DELETE CASCADE,
  "access_token" TEXT,
  "refresh_token" TEXT,
  "id_token" TEXT,
  "access_token_expires_at" TIMESTAMPTZ,
  "refresh_token_expires_at" TIMESTAMPTZ,
  "scope" TEXT,
  "password" TEXT,
  "created_at" TIMESTAMPTZ DEFAULT CURRENT_TIMESTAMP NOT NULL,
  "updated_at" TIMESTAMPTZ NOT NULL
);

CREATE TABLE "verification" (
  "id" TEXT NOT NULL PRIMARY KEY,
  "identifier" TEXT NOT NULL,
  "value" TEXT NOT NULL,
  "expires_at" TIMESTAMPTZ NOT NULL,
  "created_at" TIMESTAMPTZ DEFAULT CURRENT_TIMESTAMP NOT NULL,
  "updated_at" TIMESTAMPTZ DEFAULT CURRENT_TIMESTAMP NOT NULL
);

CREATE INDEX "session_user_id_idx" ON "session" ("user_id");

CREATE INDEX "account_user_id_idx" ON "account" ("user_id");

CREATE INDEX "verification_identifier_idx" ON "verification" ("identifier");
