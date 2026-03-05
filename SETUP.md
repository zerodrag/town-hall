Here is the strategic order of operations to set up your Town Hall project. This guide focuses on the architectural milestones—what you should actually have working at the end of each step—and points you to the official documentation for the exact, up-to-date commands.

I recommend structuring this as a monorepo: a root folder containing a `backend/` directory (Rust) and a `frontend/` directory (SvelteKit). This makes type-sharing and full-stack development much easier.

### Step 1: Database & SQLx Foundation
**The End Result:** You have a local PostgreSQL server running. In your `backend/` directory, you have a Rust project configured with SQLx. You have a `.env` file containing your `DATABASE_URL`, and you can successfully write and run a database migration to create your first table (e.g., `users` or `quests`). SQLx is configured to verify your queries against the live database at compile time.

**Where to look:**
*   **PostgreSQL:** Official downloads or use a local Docker container (`docker run postgres`).
*   **SQLx:** Search for the **SQLx GitHub repository**. Read the README section on `sqlx-cli` (for migrations) and how to set up the `PgPool`.

### Step 2: The Axum Backend Core
**The End Result:** Your Rust project boots up an HTTP server on a local port (e.g., `localhost:8080`). It takes the SQLx database pool you created in Step 1 and injects it into Axum's application state. You can open your browser, hit `localhost:8080/health`, and see a basic "OK" response confirming the server and database are connected.

**Where to look:**
*   **Axum:** Search for **docs.rs axum**. Look at the main examples, specifically how to configure `Router`, handle basic `GET` requests, and use `State` to pass your database pool to your handlers.

### Step 3: SvelteKit & Tailwind CSS Shell
**The End Result:** In your `frontend/` directory, you have a running Node.js/Bun project serving a web page on a different port (usually `localhost:5173`). File-based routing is working (creating `src/routes/quests/+page.svelte` creates a `/quests` page). Tailwind CSS is fully integrated, meaning you can add a class like `bg-blue-500 text-white` to a `div` and see the styling applied immediately when you save.

**Where to look:**
*   **SvelteKit:** Go to the **SvelteKit documentation** and find the "Creating a project" section.
*   **Tailwind CSS:** Go to the **Tailwind CSS documentation**, navigate to "Framework Guides", and follow the specific, up-to-date guide for SvelteKit.

### Step 4: The UI Component Library (shadcn-svelte)
**The End Result:** Your SvelteKit project is configured with a `components.json` file and path aliases (like `$lib/components`). You can use the shadcn CLI to inject a raw, unstyled component—like a Button or a Dialog—directly into your codebase. You can import that component into a page, render it, and it looks clean and minimalistic right out of the box, matching your Tailwind configuration.

**Where to look:**
*   **shadcn-svelte:** Go to the **shadcn-svelte documentation** and follow the "Installation" guide for SvelteKit.

### Step 5: The Type Bridge (Typeshare)
**The End Result:** You have Rust structs (like `Quest`, `User`, `Solution`) in your backend annotated with `#[typeshare]`. You have a script or a single terminal command that scans your `backend/` folder and outputs a single `types.ts` file directly into your SvelteKit's `frontend/src/lib/` directory. Your Svelte pages can now import and use these TypeScript interfaces when making `fetch` calls to your Axum API.

**Where to look:**
*   **Typeshare:** Search for the **1Password typeshare GitHub repository**. Look at the README for instructions on installing the CLI tool and how to use the `#[typeshare]` macro in Rust.

### Step 6: Authentication & Sessions
**The End Result:** You have registered a Developer Application in your GitHub account to get a Client ID and Secret. On the backend, Axum handles a `/auth/github` route that redirects the user to GitHub. GitHub redirects back to your Axum server, which uses the `oauth2` crate to exchange the code for user details. Finally, `axum-login` kicks in to create a secure, encrypted cookie-based session, saving the user to PostgreSQL. Your frontend can now ask the backend "Who am I?" and get a strictly typed user object back.

**Where to look:**
*   **GitHub OAuth:** Search for **GitHub Docs building an OAuth app**.
*   **oauth2:** Search for **docs.rs oauth2** for the specific token exchange logic.
*   **axum-login:** Search for the **axum-login GitHub repository**. Look at their `oauth2` examples. They have excellent, full-stack examples showing exactly how to integrate Axum, SQLx, and OAuth.
