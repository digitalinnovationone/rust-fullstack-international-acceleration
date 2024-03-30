# International Acceleration With Rust Fullstack Development

This international acceleration has been presented live and can be acessed via this link: https://web.dio.me/acceleration/aceleracao-internacional-rust-full-stack-development/

![Demo video of the project's frontend](./readme/demo.mp4)

## About

In this repository is demoed fullstack web development using the Rust language.

For the backend side, the [Rocket](https://rocket.rs/) framework has been used. Its API closely resembles slim frameworks such as [FastAPI](https://fastapi.tiangolo.com/) or [ExpressJS](https://expressjs.com/). Other backend options include [Axum](https://github.com/tokio-rs/axum/) and [Actix Web](https://actix.rs/), which should be preferred for production usage over Rocket. The reason Rocket has been chosen for this presentation can be found [on the introductory slide](./slides/main.pdf).

On the frontend side, the [Leptos](https://leptos.dev/) framework has been used. Its API closely resembles [React](https://react.dev/), featuring macros that provide a mixed language such as JSX/TSX, but for Rust. Components can also be defined as functions that hold both logic and appearance code.

Leptos is served via either Axum or Actix (the latter in our case). This means that a Rust Fullstack application could (and should) be contained in a single binary. This has not been done here to maximize the acceleration's reach; if that had been applied, folks interested solely on Rust's backend capabilities would need to setup the frontend repository as well.  

## Dependencies

- Rust (v1.76.0 Nightly from November/2023 has been tested and guaranteed to work);
    - Nightly is mostly necessary as of Leptos 0.6;
- Docker and Docker Compose for database;
    - This can be skipped if you already have a database running, just keep in mind to update `backend/.env` and `backend/Rocket.toml` accordingly;
- Diesel CLI (with Postgres support):
    - Depends on your system having the libpg library installed. On ArchLinux, it's provided by the `postgresql-libs` package;
    ```sh
    cargo install diesel_cli --no-default-features --features postgres
    ```
- WASM Rust build target (for Leptos CSR features):
    ```sh
    rustup target add wasm32-unknown-unknown 
    ```
- Cargo Leptos (for compiling both server-side and client-side binaries):
    ```sh
    cargo install cargo-leptos 
    ```

## Getting Started

### Backend

Before starting the backend server, assure that a database is running and that its connection is reflected on both [backend's dot env](backend/.env) and [backend's Rocket config](backend/Rocket.toml) files. By default, they're set to the connection that can be started running this on the backend folder:

```sh
docker compose up
```

Having a database set up, run the migrations with:

```sh
diesel migration run
```

With all that, you're free to use the following endpoints:

```http
GET /components/
GET /components/<id>/
POST /components/
    {
        "manufacturer": "string",
        "model": "string",
        "slot": "Cpu|Gpu|Memory|Storage",
        "price": 10,
    }

GET /rigs/
POST /rigs/
    {
        "name": "string",
        "components": [
            "id1",
            "id2",
            ...
        ]
    }
```

### Frontend

Spinning up the frontend is as easy as running:

```sh
cargo leptos watch --hot-reload
```

## Tips And Tricks

### Creating SQL Migrations based off your `schema.rs`

```sh
diesel migration generate
    --diff-schema # Uses your `schema.rs` as the source
    --locked-schema # Prevents diesel_cli from modifying your schema
    "desired_migration_name" # Substitute for a short description of what your migration does 
```

