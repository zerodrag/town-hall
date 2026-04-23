# CLAUDE.md

This file provides guidance to Claude Code (claude.ai/code) when working with code in this repository.

## Project Overview

Town Hall connects developers with "too many tokens to burn" to people who need things done. Villagers submit Quests, Adventurers claim and complete them. See `ROADMAP.md` for full domain terminology and feature roadmap.

Monorepo: Rust backend API + SvelteKit frontend. No tests exist yet in either half.

## Development Commands

### Backend (Rust)
```bash
cd backend
cp .env.example .env   # then fill in DATABASE_URL, GH_CLIENT_ID, GH_CLIENT_SECRET
cargo run               # runs on 0.0.0.0:3000, /health endpoint
cargo check             # fast compile check
```
- Uses `dotenvy` to load `.env` at startup
- On startup: runs migrations, generates TypeScript types to `frontend/src/lib/backend/generated-types.ts`
- Set `SQLX_OFFLINE=true` for IDE rust-analyzer (configured in `.zed/settings.json`)
- Rust formatting: `max_width = 120` (see `backend/rustfmt.toml`)

### Frontend (SvelteKit + Bun)
```bash
cd frontend
bun i            # Install dependencies
bun run dev      # Dev server on localhost:5173
bun run build    # Production build
bun run check    # Type-check with svelte-check
bun run lint     # ESLint + Prettier check
bun run format   # Format with Prettier
```

### SQL Linting
Root `.sqruff` configures sqruff with `dialect = postgres` for migration files.

## Architecture

### Backend (`backend/src/`)
- **Axum 0.8** with stateful router (`AppState` holds `PgPool`, OAuth client, URLs)
- **SQLx** with PostgreSQL — compile-time checked queries via `.sqlx/` offline metadata
- **Specta** collects types annotated `#[derive(specta::Type)]` and exports TypeScript at startup
- **GitHub OAuth2** with PKCE flow via `oauth2` crate
- **Sessions**: PostgreSQL-backed via `tower-sessions-sqlx-store`, `SameSite::Lax`
- **Validation**: `validator` crate with custom `NormValidJson` extractor that normalizes then validates

Routes (defined in `router.rs`):
```
GET  /                        → hello_world
GET  /health                  → health
GET  /users/{id}              → user::get
GET  /users/me                → user::get_me (session-based)
GET  /users/resolve/{handle}  → user::resolve_handle_to_id
POST /quests                  → quest::create (session-based)
GET  /quests/{id}             → quest::get (draft quests visible only to owner)
PATCH /quests/{id}            → quest::update (session-based, owner only)
GET  /auth/github             → auth::github_login (initiates OAuth)
GET  /auth/github/callback   → auth::github_callback (exchanges code, upserts user)
GET  /auth/logout             → auth::logout
```

Key patterns:
- i64 IDs are serialized as strings (`serde_as(as = DisplayFromStr)`) for JS BigInt safety; specta exports them as `string`
- `NormValidJson<T>` is a custom Axum extractor: deserializes JSON, calls `.normalize()`, then `.validate()`
- Auth is session-based — `resolve_me_id(&session)` returns user ID or `BackendError::Unauthorized`
- `BackendError` enum with `IntoResponse` maps domain errors to HTTP status codes

### Frontend (`frontend/src/`)
- **SvelteKit 5** with Svelte 5 runes (`$state`, `$derived`, `$props` — not old stores)
- **SvelteKit adapter-node** for production deployment; `runes: true` enforced for all `.svelte` files
- **TailwindCSS 4** via `@tailwindcss/vite`, Catppuccin Mocha dark theme (CSS variables in `routes/layout.css`)
- **shadcn-svelte** components (maia style) in `src/lib/components/ui/`, built on `bits-ui`
- **Bun** as package manager (not npm/pnpm)

API layer in `src/lib/backend/`:
- `common.ts` — `BACKEND_URL` constant and `fetchBackend()` helper (sets `credentials: 'include'`)
- `quest.ts` / `user.ts` — typed API call functions using `fetchBackend`
- `generated-types.ts` — auto-generated from Rust specta types (do not edit manually)

File-based routing under `src/routes/`:
- `+layout.ts` — loads current user via `getUserMe`; returns `{ me }` or `{ user: null }`
- `quest/[id]/[title]/` — quest view page (slug-verified, redirects if mismatch)
- `quest/[id]/[title]/edit/` — quest editor (auth guard: 401/403/404)
- `user/[id]/[handle]/` — user profile (slug-verified)
- `user/me/` — redirects to `/user/{userId}/{handle}`

Auth flow: cookie-based server sessions. Frontend determines auth state by success/failure of `GET /users/me`. No JWT or client-side auth store.

### Database Schema
- **users**: `user_id` (identity PK), `github_id` (unique), `handle`, `email` (unique), `created_at`
- **quests**: `quest_id` (identity PK), `poster_id` FK→users, `title` (1-100 chars, CHECK constraint), `summary`, `details`, `techs` (text[]), `status` enum (draft/ongoing/solved), `created_at`
- Session table created at runtime by `tower-sessions-sqlx-store`

### Formatting Conventions
- Frontend: Prettier with `singleQuote: true`, `trailingComma: 'none'`, `printWidth: 120`, `@ianvs/prettier-plugin-sort-imports`
- Frontend: ESLint with `svelte.configs.recommended` + `prettier`, `no-undef` off for TS
- Backend: `rustfmt.toml` sets `max_width = 120`