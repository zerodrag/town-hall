# Good First Project

*The cookie bucket collection.*

GFP is a website that connect first time (vibe-)coders, who would otherwise make another TODO app, to actual projects with real value.

## Usage

### Backend
Prerequisites: [Rust](https://rustup.rs/)
```bash
cd backend
cargo run
```

Accessible at 0.0.0.0:3000(/health) by default

Requires setting up a PostgreSQL database under `backend/.env` according to [.env.example](backend/.env.example)

Generates types [here](frontend/src/lib/backend/generated-types.ts) on server start

### Frontend
Prerequisites: [Bun](https://bun.com/)
```bash
cd frontend
bun i
bun run dev
```
Accessible at 0.0.0.0:5173 by default 

Requires backend to be running

## Inspiration
[Modrinth](https://github.com/modrinth/code) for frontend design 

## Credits
[shadcn-svelte](https://github.com/huntabyte/shadcn-svelte)

## AI usage
Large language models were used in development on read-only mode. All code was written by hand.
