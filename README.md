# Krabby Chat WebSocket Server.

This repository contains the WebSocket server/service for the Krabby `chat` implementation.

## Core Features

- **High-Performance WebSockets**: Built on Axum and Tokio for scalable, non-blocking real-time communication.

- **Dynamic Multi-layer configuration system**: Powered by the `config` crate, supporting TOML and Environment Variables.

- **Structured JSON Logging**: Comprehensive visibility with `tracing` and `tracing-subscriber`.

- **Standardized Developer Workflow**: Integrated with Husky and Commitlint for high-quality, conventional contributions.

## Setup & Execution

### 1. Core Prerequisites

- [Rust](https://www.rust-lang.org/tools/install)(latest stable)

- [Node.js](https://nodejs.org/en/download/)(and [Bun](https://bun.sh/)) - for contribution standards enforcement)

### 2. Running the Server

*Ensure to have installed `cargo-watch`.*

```shell
cargo install cargo-watch
```

To start the server in development mode(auto-reload enabled), simply run:

```shell
cargo dev
```

> `cargo-watch` handles the server/project reloads on-save. See `.cargo/config.toml` for reference on the `dev` command.

*Note: The `dev` command is an alias for `cargo watch`. If you are on WSL and reload doesn't trigger, proceed to use the polling command option(also see `.cargo/config.toml` for reference on that).*

### 3. Setting up to ensure contribution standards

Following the architectural patterns of the Krabby ecosystem, this project uses a cross-language workflow to ensure:

- Standardized commit messages across all contributors.

- Automatic pre-commit checks (formatting, linting, compilation).

- Pre-push checks enforcement.

> The project core is pure Rust. The `Node.js` integration only introduces the packages needed for enforcing code/contribution standards.
>
> P.S: The preferred `Node.js` package manager is `Bun`.

To integrate the `Husky` and `Commitlint` setup into your local workflow:

```shell
bun install
```

## Project Config Setup

The project uses a highly flexible configuration pattern powered by the `config` crate.

### Loading Order(Arranged in increasing order of overriding authority):

1. **Base Config**: `config/base.toml` (Default values).

2. **Environment Config Overrides**: `config/{APP__ENV}.toml` (e.g., `development.toml`, `production.toml`).

3. **Local Overrides**: `config/local.toml` 

4. **Environment Variables**: Prefixed with `APP__`.

### Mapping Rule for Environment Variables

`__`(double underscore) is used as a separator to map to nested TOML sections.

**Syntax:** `APP__<SECTION>__<FIELD>=value`

**Example:**

To override the server port in `base.toml`:

```toml
[server]
port = 8001
```

Set the environment variable:

`APP__SERVER__PORT=9000`

### Mandatory Sections

The `validate()` method ensures the following sections are correctly populated at startup:

- `app`: Basic metadata.

- `server`: Host, Port, and Request Timeouts.

## Environment Variables Files

The project uses several `.env` files to manage environment-specific configurations. 

- `.env`: The default environment file. Its primary function is controlling environment selection (e.g., `APP__DEPLOY__ENV=development`).

- `.env.development`: Contains configuration overrides specifically for local development.

- `.env.staging`: Contains configuration overrides specifically for staging/pre-production environment.

- `.env.production`: Contains sensitive production-only settings.

## Testing

The project maintains reliability through comprehensive testing.

### 1. Unit Tests

Located within the source files (e.g., `src/utils/load_config.rs`). 

**Run unit tests:**

```shell
cargo test --lib
```

### 2. Integration Tests (Roadmap)

All integration tests are expected to verify end-to-end WebSocket messaging flows, including connection upgrades, message broadcasting, and protocol adherence.

### 3. How to add new tests

- **Unit Tests**: Add a `#[cfg(test)]` block at the end of your module. Followed by the respective tests as intended.

E.g.

```rust
#[cfg(test)]
mod tests {
    #[test]
    fn test_something() {
        assert!(true);
    }
}
```

## Reliability & Robustness

- **Asynchronous Concurrency**: Built on Tokio to handle thousands of concurrent WebSocket connections efficiently.

- **Fast Initialization**: Robust config validation prevents the server from starting in an invalid state.

- **Structured Observability**: Tracing logs are JSON-formatted by default for easy consumption by log aggregators in production.

## Operating System Notes (WSL)

If you are developing on **WSL**, file system events might not trigger `cargo watch`. Use the polling variant of `cargo dev` defined in `.cargo/config.toml`.

Cheers!!! 🍻
