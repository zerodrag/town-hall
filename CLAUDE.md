# CLAUDE.md

This file provides guidance to Claude Code (claude.ai/code) when working with code in this repository.

## Project Overview

Town Hall connects developers with "too many tokens to burn" to people who need things done. The platform consists of a Rust backend (API server) and a SvelteKit frontend.

## Development Commands

### Backend (Rust)
```bash
cd backend
cargo run
```
- Runs on `0.0.0.0:3000` with a `/health` endpoint
- Requires `DATABASE_URL`, `GH_CLIENT_ID`, and `GH_CLIENT_SECRET` environment variables
- On startup, automatically runs migrations and generates TypeScript types to `frontend/src/lib/backend/generated-types.ts`

### Frontend (SvelteKit + Bun)
```bash
cd frontend
bun i          # Install dependencies
bun run dev    # Start dev server
bun run build  # Production build
bun run check  # Type-check with svelte-check
bun run lint   # Run ESLint + Prettier
bun run format # Format with Prettier
```

## Architecture

### Backend (`backend/src/`)
- **Axum** web framework with tower middleware (sessions, CORS)
- **SQLx** with PostgreSQL for database access
- **Specta** generates TypeScript types from Rust structs at runtime — types auto-export to `frontend/src/lib/backend/generated-types.ts`
- **GitHub OAuth** for authentication via `oauth2` crate
- **Session storage**: PostgreSQL via `tower-sessions-sqlx-store`

Handlers are organized by domain:
- `handlers/auth.rs` — GitHub OAuth flow (login, callback, logout)
- `handlers/quest.rs` — Quest CRUD operations
- `handlers/user.rs` — User profile and session resolution
- `handlers/helper.rs` — Shared utilities (e.g., `resolve_current_user_id`)

### Frontend (`frontend/src/`)
- **SvelteKit 5** with Svelte 5 runes (use `$state`, `$derived`, etc. — not old stores)
- **TailwindCSS 4** with `@tailwindcss/vite` plugin
- **Bun** as package manager (not npm/pnpm)
- Svelte 5 component style: `.svelte` files with `<script>`, markup, and optional `<style>`

File-based routing under `src/routes/`:
- `+layout.svelte` / `+layout.ts` — root layout
- `quest/[id]/[title]/edit/` — quest editor page
- `user/[id]/[handle]/` — user profile page
- `user/me/` — current user's page

API clients in `src/lib/backend/`:
- `quest.ts` — quest CRUD calls
- `user.ts` — user API calls
- `generated-types.ts` — auto-generated from Rust specta types

UI components in `src/lib/components/ui/` are shadcn-style primitives (button, card, dialog, etc.).
