# PLAN: Plain MVP (Axum + static frontend)

Objective: ship a demo with the smallest stack—Axum only on the backend, plain static files on the frontend, no DB, no OAuth.

## Step 1 — Axum core (backend/)
- Axum binary with `tokio`.
- Dependencies: `axum = "0.8"`, `tokio = { version = "1", features = ["full"] }`. Add `tower-http = { version = "0.6", features = ["fs"] }` if you want to serve static files.
- Routes to start:
  - `GET /health` → `200 OK` with body "ok".
  - `GET /quests` → return an in-memory list of quests (hardcoded Vec).
  - `POST /quests` (optional) → accept JSON, push into in-memory Vec, return created item.
  - `GET /` → either "hello" or serve `frontend/` via `ServeDir`.
- Bind to `0.0.0.0:3000` and confirm `/health` works.

## Step 2 — Static frontend (frontend/)
- No build tools. Create `frontend/index.html` and `frontend/app.js` (optional).
- In the page, call `/health` and `/quests`, render results, and show a simple form to post a quest (if you keep `POST /quests`).
- Style can be bare HTML/CSS; keep it minimal.

## Step 3 — UX slices (mirror README ideas, still in-memory)
- Quests list view: cards with title, description, status badge (open/claimed/approved). Buttons: "Claim" (toggles claimed), "View" (shows details).
- Quest detail view: shows quest info plus a Solutions section listing links/notes; a small form to add a solution; badge to mark one solution as "Approved".
- Claims UX: when claimed, show the claimer name next to the quest; allow "Unclaim".
- Council/Disputes stub: a simple "Reports" list with status chips (open/under-review/resolved) and a button to "File report" that appends a new row.

## Step 4 — Next after the MVP
- Swap the in-memory store for a real DB only after the static demo works.
- Add GitHub OAuth later; keep current flows unauthenticated for now.
- Bring back a framework (e.g., SvelteKit) only if/when the static page becomes painful.
