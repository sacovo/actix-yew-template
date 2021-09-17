# Webapp in Rust

Template for a web app completly in rust. The server is created with actix-web, the frontend with yew.

Put shared structs and functionality into the root crate, app or server specific stuff into the corresponding app or server crates.

Run `cargo watch -w server -w src -x "run"` in the root folder of the repository to start the server and restart on changes. In another shel run `trunk watch` inside the app folder to rebuild the wasm code on changes.

Features:
- actix is setup to server the static files produced by trunk
- a service worker for pwa functionality is included
