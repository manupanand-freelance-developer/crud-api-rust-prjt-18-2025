# CRUD API with Rust and Actix Web

A simple CRUD (Create, Read, Update, Delete) REST API built with Rust using the Actix Web framework and PostgreSQL as the database[1].

## Features

-   RESTful API endpoints for basic CRUD operations
-   PostgreSQL integration with `sqlx`
-   JSON serialization with `serde`
-   User authentication support with `jsonwebtoken` and password hashing using `argon2`
-   Environment variable configuration with `dotenv`
-   CORS support with `actix-cors`
-   Logging with `env_logger`
-   UUID support for unique identifiers
-   Date and time handling with `chrono`

## Getting Started

### Prerequisites

-   Rust (latest stable version recommended)
-   PostgreSQL database (remote or local)
-   `cargo` package manager (comes with Rust)
-   `sqlx-cli` for database migrations (optional but recommended)

### Setup

1.  Clone the repository:

    ```
    git clone <repository-url>
    cd crud-api
    ```

2.  Create a `.env` file in the root directory with your PostgreSQL connection URL:

    ```
    DATABASE_URL=postgresql://user:password@your-postgres-host/database_name
    ```

3.  Install dependencies and build the project:

    ```
    cargo build
    ```

4.  Run database migrations (if you use `sqlx` migrations):

    ```
    sqlx migrate run
    ```

5.  Start the server:

    ```
    cargo run
    ```

The API will be running at `http://localhost:8000`

## API Endpoints

| Method | Endpoint            | Description              |
| ------ | ------------------- | ------------------------ |
| GET    | `/api/healthchecker` | Health check endpoint    |
| GET    | `/api/items`        | Retrieve all items       |
| POST   | `/api/items`        | Create a new item        |
| GET    | `/api/items/{id}`   | Retrieve an item by ID  |
| PATCH  | `/api/items/{id}`   | Update an item by ID    |
| DELETE | `/api/items/{id}`   | Delete an item by ID    |

*(Replace `items` with your actual resource name, e.g., `todos` or `users`.)*

## Dependencies

Key crates used in this project:

-   `actix-web` — Web framework for building HTTP servers
-   `actix-cors` — Cross-Origin Resource Sharing support
-   `serde` and `serde_json` — Serialization and deserialization of JSON
-   `sqlx` — Async PostgreSQL driver and ORM
-   `uuid` — For generating UUIDs
-   `chrono` — Date and time handling
-   `jsonwebtoken` — JWT token creation and validation
-   `argon2` — Password hashing
-   `dotenv` — Manage environment variables
-   `env_logger` — Logging support

## Development Tips

Use `cargo-watch` for hot reloading during development:

```
cargo install cargo-watch
cargo watch -q -c -w src/ -x run

```
### Configure logging by setting the environment variable before running:

```export RUST_LOG=actix_web=info```


## License

This project is licensed under the MIT License.