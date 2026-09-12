# rustweb demo — TaskBoard

Full demo web app built on `rustweb 1.1` (Virtual DOM, SSR/hydration, router, headless UI).

## Run (native SSR preview)

```bash
cargo run
```

Prints SSR HTML and verifies hydration.

## Run in browser (wasm)

Requires `trunk` or `wasm-pack`:

```bash
cargo install trunk --locked
trunk serve --open
# or
wasm-pack build --target web
python3 -m http.server
```

Build output mounts into `#app` via `src/main.rs` wasm entry.

## Stack

- `rustweb-core` / `macro` / `dom` / `router` / `ssr` / `ui` — all `1.1`
- `rustweb-ui` primitives: `Button`/`Input`/`Dialog`/`Tabs` with `Theme`
- Router: `/`, `/settings`, `settings/profile` with guards/lazy demo
