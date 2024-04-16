# Tools to install

## Rust Toolchain
```sh
rustup toolchain install nightly-2023-11-26-x86_64-unknown-linux-gnu
rustup default nightly-2023-11-26-x86_64-unknown-linux-gnu
rustup target add wasm32-unknown-unknown
```

## Diesel and Leptos CLIs
```sh
cargo install diesel_cli --no-default-features --features postgres
cargo install cargo-leptos
```

# Project Dependencies

## Backend Dependencies
```sh
cargo add diesel --features=postgres,uuid
cargo add diesel-derive-enum --features=postgres
cargo add rocket --features=json
cargo add rocket_db_pools --features=diesel_postgres
cargo add serde --features=derive
cargo add uuid --features=v4,serde
```

## Frontend Dependencies
```sh
cargo add wasm-bindgen@0.2.92
cargo add thaw --git https://github.com/thaw-ui/thaw
cargo add leptos-use@0.10.4
cargo add leptos-struct-table@0.9.1
cargo add icondata@0.3.0
cargo add reqwest@0.11.25 --no-default-features --features=json
cargo add serde@1.0.197 --features=derive
cargo add strum@0.26.2 --features=derive
```

# Handy commands
- Setup Diesel on backend directory
  ```sh
  diesel setup
  ```
- Create new migration named <migration-name>
  ```sh
  diesel migration generate --diff-schema <migration-name> --locked-schema
  ```
- Create a new Leptos project from template
  ```sh
  cargo leptos new --git https://github.com/leptos-rs/start
  ```
- Run Leptos with hot-reload
  ```sh
  cargo leptos watch --hot-reload
  ```
