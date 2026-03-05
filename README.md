# Chat Example

This is an example application which demonstrates an end-to-end encrypted cross-platform chat application which primarily uses HTML via Tera templating for UI AND serving application data. 
- Application data is served via HTML as a data attribute encoded as a base64 blob and decrypted via the WebCrypto API which does the majority of end-to-end encryption heavy lifting. 
- The application is available on desktop using Tauri webview and on mobile using PWAs (progressive web applications) which utilizes service workers & the IndexedDB API for offline interacivity.

## Usage

Prerequisities
- [Rust](https://rust-lang.org/tools/install/)
- [Docker](https://docs.docker.com/engine/install/)

Instructions
1. Clone this repository
2. Copy `.env.example` to `.env` & set `POSTGRES_PASSWORD=` variable
3. Start database with `docker compose -f docker-compose.dev.yml up -d`
3. Run the application with `cargo run`
4. Go to `http://localhost:8080`

## Functionality
- Create a user account with end-to-end encryption & recovery codes
- Add another user as friend by their username & discriminator (e.g., @hyziri#0000)
- New friend requests & message notifications via websocket
- Send end-to-end encrypted messages between users
- Groups chats between multiple users

## Architecture

Serverside rendered HTML templating-based UI with HTMX interactivity & Rust Axum API backend

Device targets:
- Web browser with Tera HTML templates served via Axum API
- Desktop with Tauri
- Mobile with PWAs (Progressive web applications)

Frontend (JavaScript):
- Tera with HTML templating for UI
- HTMX for interactivty & SPA-like page navigation
- JavaScript for minor interactivity (Such as opening modals)
- Basic inline CSS for styling

Backend (Rust):
- tera for HTML templating for UI & serving data as HTML
- tokio for async runtime
- axum for API endpoints serving HTML
- tokio-postgres for database interaction
- tower-sessions for session management
- tower-sessions-redis-store for storing session information via redis

Applications
- Postgres for database
- Redis for sessions
