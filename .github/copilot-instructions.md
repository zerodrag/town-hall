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

- Build: `pnpm build`
- Preview production build: `pnpm preview`
- Type/check pass: `pnpm check`
- Lint: `pnpm lint`
- Format: `pnpm format`

There is currently no frontend test runner configured.

## High-level architecture

- This is a two-app repo: a Rust/Axum backend on `localhost:3000` and a SvelteKit frontend on `localhost:5173`.
- The product vocabulary in `README.md` matters: Villagers submit Quests, Adventurers work on them, Solutions can be approved, Claims are non-exclusive intent signals, and Councils resolve disputes.
- Backend startup in `backend/src/main.rs` loads env vars, builds `AppState`, runs SQLx migrations, configures Postgres-backed `tower-sessions`, enables CORS for the frontend origin, and mounts grouped routers under `/api` and `/auth`.
- `backend/src/state.rs` is the shared dependency composition root. It owns the Postgres pool and the GitHub OAuth client.
- The implemented end-to-end flow is authentication and current-user lookup:
  - frontend `/login` links directly to backend `/auth/github`
  - backend `/auth/github` creates PKCE + CSRF values and stores them in the session
  - `/auth/github/callback` exchanges the code with GitHub, upserts the user into `users`, stores the internal `user_id` in the session, and redirects to frontend `/dashboard`
  - `/api/users/me` reads `user_id` from the session and returns the logged-in user
- The frontend is still a thin SvelteKit shell: global layout, login/dashboard routes, Tailwind setup, and UI wrappers in `frontend/src/lib/components/ui/`.

## Key conventions

- Backend handlers live in `backend/src/handlers/*.rs`, are re-exported from `backend/src/handlers.rs`, and are wired together in `backend/src/router.rs`. Follow that structure instead of registering ad hoc handlers in `main.rs`.
- Reuse `AppState` for shared dependencies. Do not create extra database pools or OAuth clients inside handlers.
- Prefer the existing SQLx macro style (`query_as!`, `query_scalar!`) for compile-time checked queries, and keep the `.sqlx` cache in sync when query shapes or migrations change.
- The backend uses server-side sessions stored in Postgres, not JWTs or frontend-managed auth state. Existing session keys are `csrf_token`, `pkce_verifier`, and `user_id`.
- Dev URLs are hardcoded in multiple places (`backend/src/main.rs`, `backend/src/state.rs`, `backend/src/handlers/auth.rs`, and `frontend/src/routes/login/+page.svelte`). If the frontend/backend host or port changes, update CORS, the OAuth callback URI, backend redirects, and the frontend login link together.
- Frontend formatting is opinionated: tabs, single quotes, no trailing commas, and Tailwind class sorting through Prettier. Match the existing `frontend/.prettierrc` + `frontend/eslint.config.js` setup instead of hand-formatting.
- Svelte 5 snippets are used in UI wrappers and layout delegation; when destructuring snippet params like `child({ props })`, add explicit types if inference gets fuzzy in the editor.
- Prefer the existing bits-ui wrapper components under `frontend/src/lib/components/ui/` when adding or extending navigation, menu, avatar, or button behavior.
