# Town Hall Copilot Instructions

## Build, test, and lint commands

Run commands from the owning app directory; there is no root-level task runner.

### Backend (`backend/`)

- Build: `SQLX_OFFLINE=true cargo build`
- Full test run: `SQLX_OFFLINE=true cargo test`
- Single test: `SQLX_OFFLINE=true cargo test <test_name>`
- Formatting check: `cargo fmt --check`

The backend uses SQLx compile-time checked macros and keeps offline metadata in `backend/.sqlx`. The editor config in `.zed/settings.json` sets `SQLX_OFFLINE=true`, so keep that environment variable set for local Rust builds and tests.

### Frontend (`frontend/`)

- Build: `pnpm build`
- Lint: `pnpm lint`
- Type/check pass: `pnpm check`

There is currently no frontend test runner configured.

## High-level architecture

- This repo is a two-app setup: a Rust/Axum backend on `localhost:3000` and a SvelteKit frontend on `localhost:5173`.
- The domain language in `README.md` matters when naming features: Villagers submit Quests, Adventurers work on them, Solutions can be approved, and Councils resolve disputes.
- Backend startup in `backend/src/main.rs` loads env vars, creates `AppState`, runs SQLx migrations, configures Postgres-backed `tower-sessions`, enables CORS for the frontend origin, and mounts grouped routers under `/api` and `/auth`.
- `backend/src/state.rs` is the composition root for shared resources. It owns both the Postgres pool and the GitHub OAuth client.
- The implemented end-to-end flow today is authentication and current-user lookup:
  - frontend `/login` links directly to backend `/auth/github`
  - backend `/auth/github` creates PKCE + CSRF values and stores them in the session
  - `/auth/github/callback` exchanges the code with GitHub, upserts the user into `users`, stores the internal `user_id` in the session, and redirects to frontend `/dashboard`
  - `/api/users/me` reads `user_id` from the session and returns the logged-in user
- `backend/src/handlers/quest.rs` exists, but quest routes are not mounted in `backend/src/router.rs`, and the checked migrations only create `users`. The auth/user slice is further along than the quest feature set.
- The frontend is still a thin SvelteKit shell: global layout + Tailwind setup, a login page, and an empty dashboard server route. Keep backend ownership of auth/session state in mind when extending it.

## Key conventions

- Backend handlers live in `backend/src/handlers/*.rs`, are re-exported from `backend/src/handlers.rs`, and are wired together in `backend/src/router.rs`. Follow that structure instead of registering ad hoc handlers in `main.rs`.
- Reuse `AppState` for shared dependencies. Do not create extra database pools or OAuth clients inside handlers.
- Prefer the existing SQLx macro style (`query_as!`, `query_scalar!`) for compile-time checked queries, and keep the `.sqlx` cache in sync when query shapes or migrations change.
- The backend uses server-side sessions stored in Postgres, not JWTs or frontend-managed auth state. Existing session keys are `csrf_token`, `pkce_verifier`, and `user_id`.
- Dev URLs are hardcoded in multiple places (`backend/src/main.rs`, `backend/src/state.rs`, `backend/src/handlers/auth.rs`, and `frontend/src/routes/login/+page.svelte`). If the frontend/backend host or port changes, update CORS, the OAuth callback URI, backend redirects, and the frontend login link together.
- Frontend formatting is opinionated: tabs, single quotes, no trailing commas, and Tailwind class sorting through Prettier. Match the existing `frontend/.prettierrc` + `frontend/eslint.config.js` setup instead of hand-formatting.
