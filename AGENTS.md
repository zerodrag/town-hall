# Town Hall

Monorepo: `backend/` (Rust/Axum) + `frontend/` (SvelteKit/Bun).

## Development

### Backend
```bash
cd backend
cargo run                              # starts server at 0.0.0.0:3000
```

Requires `.env` with `DATABASE_URL`, `GH_CLIENT_ID`, `GH_CLIENT_SECRET`.

**SQLx offline mode**: Run `cargo sqlx prepare` after adding/modifying queries to update `.sqlx/` cache. Rust-analyzer uses `SQLX_OFFLINE=true` via `.zed/settings.json`.

**Type generation**: Backend exports TypeScript types to `frontend/src/lib/backend/generated-types.ts` via Specta on every build.

### Frontend
```bash
cd frontend
bun i                                  # install deps
bun run dev                            # starts dev server at 0.0.0.0:5173
bun run check                          # typecheck
bun run lint                           # prettier check + eslint
bun run format                         # auto-format
```

## Architecture

- Backend: Axum with PostgreSQL (sqlx), session store via `tower-sessions-sqlx-store`, GitHub OAuth
- Frontend: SvelteKit 5 (runes mode) with TailwindCSS v4, adapter-node
- Migrations: SQL files in `backend/migrations/`, auto-run on startup via `sqlx::migrate!()`

### Backend Routes
```
GET  /                        → hello_world
GET  /health                  → health
GET  /users/{id}              → user::get
GET  /users/me                → user::get_me (session-based)
GET  /users/resolve/{handle}  → user::resolve_handle_to_id
POST /quests                  → quest::create (session-based)
GET  /quests/{id}             → quest::get (draft quests visible only to owner)
PATCH /quests/{id}            → quest::update (session-based, owner only)
GET  /discover/quests         → quest::discover (search quests with pagination)
GET  /auth/github             → auth::github_login (initiates OAuth)
GET  /auth/github/callback   → auth::github_callback (exchanges code, upserts user)
GET  /auth/logout             → auth::logout
```

### Key Patterns
- i64 IDs serialized as strings for JS BigInt safety
- `NormValid<Json<T>>` and `NormValid<Query<T>>` extractors: deserialize, normalize, validate
- Auth via `resolve_me_id(&session)` returns user ID or `BackendError::Unauthorized`