# Town Hall Copilot Instructions

## Build, test, and lint commands

Run commands from the owning app directory; there is no root-level task runner.

### Backend (`backend/`)

- Build: `SQLX_OFFLINE=true cargo build`
- Full test run: `SQLX_OFFLINE=true cargo test`
- Single test: `SQLX_OFFLINE=true cargo test <test_name>`
- Formatting check: `cargo fmt --check`

Keep `SQLX_OFFLINE=true` set for local Rust builds and tests. The backend uses SQLx compile-time checked macros and keeps offline metadata in `backend/.sqlx`.

### Frontend (`frontend/`)

- Build: `bun run build`
- Preview production build: `bun run preview`
- Type/check pass: `bun run check`
- Lint: `bun run lint`
- Format: `bun run format`

The frontend scripts are Bun-based (`bunx --bun ...` in `frontend/package.json`), and there is currently no frontend test runner or single-test command configured.

## High-level architecture

- This is a two-app repo: a Rust/Axum backend on `localhost:3000` and a SvelteKit frontend on `localhost:5173`.
- The product vocabulary in `README.md` matters: Villagers submit Quests, Adventurers work on them, Solutions can be approved, Claims are non-exclusive intent signals, and Councils resolve disputes.
- Backend startup in `backend/src/main.rs` loads env vars and CLI args, generates `frontend/src/lib/backend/generated-types.ts` from Rust Specta types, builds `AppState`, runs SQLx migrations, configures Postgres-backed `tower-sessions`, enables CORS for the frontend origin, and mounts grouped routers from `backend/src/router.rs`.
- `backend/src/state.rs` is the shared dependency composition root. It owns the Postgres pool and the GitHub OAuth client, so handlers should reuse `AppState` instead of constructing their own clients or pools.
- The implemented end-to-end auth flow is:
  - the frontend login button links directly to `${BACKEND_URL}/auth/github`
  - backend `/auth/github` creates PKCE + CSRF values and stores them in the session
  - `/auth/github/callback` exchanges the code with GitHub, upserts the user into `users`, stores the internal `user_id` in the session, and redirects to frontend `/user/{id}/{handle}`
  - the frontend root layout fetches `/users/me` with cookies included to populate the current user
- The current Quest flow is:
  - the navbar "New Quest" dialog posts `CreateQuestRequest` to `/quests`
  - successful creation redirects to `/quest/{id}`, and frontend loaders immediately normalize that to the canonical slug route
  - backend `GET /quests/{id}` hides `draft` quests from non-owners, and the frontend edit route mirrors that ownership check
  - quest creation and viewing are wired end-to-end; quest editing UI exists, but persistence is not implemented yet

## Key conventions

- Backend handlers live in `backend/src/handlers/*.rs`, are re-exported from `backend/src/handlers.rs`, and are wired together in `backend/src/router.rs`. Follow that structure instead of registering ad hoc handlers in `main.rs`.
- Prefer the existing SQLx macro style (`query_as!`, `query_scalar!`) for compile-time checked queries, and keep migrations plus the `.sqlx` cache in sync when query shapes change.
- The backend uses server-side sessions stored in Postgres, not JWTs or frontend-managed auth state. Existing session keys are `csrf_token`, `pkce_verifier`, and `user_id`.
- Frontend backend calls should go through helpers in `frontend/src/lib/backend/` and reuse `fetchBackend(..., { credentials: 'include' })` so cookie-backed auth keeps working.
- Do not hand-edit `frontend/src/lib/backend/generated-types.ts`; it is generated from backend Specta types during backend startup.
- Canonical entity URLs are enforced in frontend loaders: `/quest/[id]` redirects to `/quest/[id]/[slug]`, and `/user/[id]` redirects to `/user/[id]/[handle]`. Follow the same pattern when adding more entity pages.
- Draft quests are treated as an access-control rule, not just a display status: non-owners should get a not-found response rather than seeing draft data.
- Dev URLs are hardcoded in multiple places (`backend/src/main.rs`, `backend/src/state.rs`, `backend/src/handlers/auth.rs`, and `frontend/src/lib/backend/common.ts`). If the frontend/backend host or port changes, update CORS, the OAuth callback URI, backend redirects, and the frontend backend URL together.
- Frontend code uses Svelte 5 runes/snippets and Bits UI wrapper components under `frontend/src/lib/components/ui/`. Prefer extending those wrappers over introducing parallel component patterns.
- Match the frontend formatting config in `frontend/.prettierrc.ts`: spaces (not tabs), single quotes, no trailing commas, and Tailwind class sorting through Prettier.
