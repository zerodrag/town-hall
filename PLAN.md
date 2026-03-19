# PLAN: Frontend route skeleton (`/u` + `/q`)

Objective:
- Replace the current placeholder frontend route tree with the agreed URL structure.
- Keep short public frontend URLs (`/u`, `/u/[handle]`, `/q`, `/q/[questId]-[slug]`) and explicit non-versioned backend API routes (`/api/users`, `/api/quests`, `/api/me`).
- Do not run builds, checks, or tests as part of this task; validation will be done separately.

## Route tree to implement in `frontend/src/routes/`

```text
+page.svelte
login/+page.svelte
settings/+page.svelte

u/+page.server.ts
u/[handle]/+page.svelte
u/[handle]/quests/+page.svelte
u/[handle]/solutions/+page.svelte

q/+page.svelte
q/new/+page.svelte
q/[questId]-[slug]/+page.svelte
q/[questId]-[slug]/edit/+page.svelte
q/[questId]-[slug]/solutions/new/+page.svelte
q/[questId]-[slug]/solutions/[solutionId]/+page.svelte
q/[questId]-[slug]/reports/new/+page.svelte

guilds/+page.svelte
guilds/[guildSlug]/+page.svelte

council/cases/+page.svelte
council/cases/[caseId]/+page.svelte
```

## Current frontend files to replace or remove

- Replace `frontend/src/routes/+page.svelte` so `/` becomes a real entry page instead of the SvelteKit placeholder.
- Keep `frontend/src/routes/login/+page.svelte` as the dedicated auth entrypoint.
- Remove `frontend/src/routes/dashboard/+page.server.ts`; the first pass should not keep a separate dashboard.

## Implementation steps

### 1. Create the route skeleton

- Add the route directories and `+page.svelte` / `+page.server.ts` files listed above.
- Use lightweight placeholder content where backend data is not ready yet.
- Treat `/u/[handle]` and `/q/[questId]-[slug]` as the canonical public pages.

### 2. Add shared route helpers

Create a small helper module under `frontend/src/lib/`, for example `routing.ts` or `paths.ts`, that centralizes path generation:

- `userPath(handle: string) => /u/${handle}`
- `questPath(id: number | string, slug: string) => /q/${id}-${slug}`
- `slugifyQuestTitle(title: string) => string`
- A parser/helper for splitting `[questId]-[slug]` if route code needs the numeric ID

This keeps links and redirects consistent across the app.

### 3. Implement the self-profile shortcut

- `frontend/src/routes/u/+page.server.ts` should redirect authenticated users to their canonical `/u/[handle]` page.
- If the user is not authenticated, `/u` should redirect to `/login`.
- This replaces the need for a `/dashboard` route in the first pass.

### 4. Update top-level navigation and entry points

- `/` should become a simple entry page that points users toward quest discovery (`/q`), login (`/login`), and their own profile shortcut (`/u`) when signed in.
- Shared navigation should link to `/q`, `/u`, `/settings`, `guilds`, and `council/cases` as appropriate.
- Do not add a dashboard link.

### 5. Keep quest detail as the interaction hub

- Claims, questions, and most quest discussion should live as sections on `/q/[questId]-[slug]`.
- Dedicated nested routes are only needed for flows that deserve their own page:
  - editing a quest
  - submitting a solution
  - viewing a specific solution
  - filing a report

### 6. Preserve backend/API separation

- Frontend uses short public URLs.
- Backend stays explicit and non-versioned:
  - `/auth/github`
  - `/auth/github/callback`
  - `/auth/logout`
  - `/api/me`
  - `/api/users/:userId`
  - `/api/quests/:questId`
  - other nested resource routes under `/api/quests`, `/api/guilds`, and `/api/council`

## Backend data dependencies to call out before implementation

These are not the first implementation slice, but the frontend route plan depends on them:

- The backend currently does **not** store a stable public user handle. `users` contains `github_id`, `display_name`, and `email`, but no `handle`.
- To make `/u/[handle]` real, add a `handle` field to the user model and persist GitHub's `login` value (or another chosen public handle) during OAuth.
- `/api/me` should eventually return enough information to resolve `/u/[handle]`.
- Quest responses need a stable ID and a title. The frontend can derive the slug from the title; the ID remains the real lookup key.

## Out of scope for this slice

- Reorganizing backend Axum routes
- Implementing full data loading for every page
- Adding a dashboard
- Running `pnpm build`, `pnpm check`, or `pnpm lint`

## Notes

- Roles are contextual: the same account may be a Villager on one quest and an Adventurer on another, so there should be no separate `villager/` or `adventurer/` trees.
- `/u` is a convenience shortcut only; `/u/[handle]` is the canonical public URL.
- `/q/[questId]-[slug]` uses the numeric ID as the real identifier and the slug as a readability aid.
