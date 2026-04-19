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