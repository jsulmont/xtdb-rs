# Rust Driver for XTDB 2.x

`xtdb-rs` is a Rust driver designed for interacting with XTDB 2.x. This project focuses on providing a seamless way to execute XTQL queries and handle transactions within Rust code, ensuring complete parsing of XTQL expressions with meaningful error messages.

## Features

- **Execute Queries**: Directly execute XTQL queries and receive responses in JSON format.
- **Transaction Handling**: Ability to handle transactions (currently under development).
- **XTQL Integration**: Embed XTQL in your Rust code for efficient query operations.
- **Error Handling**: Provides comprehensive error handling with meaningful error descriptions.

## Current Limitations

- **Incomplete**: Due to time constraints, the project is currently incomplete and lacks comprehensive tests.
- **Query Focused**: Currently, the driver supports only query operations as per the [XTDB XTQL Query Reference](https://docs.xtdb.com/reference/main/xtql/queries.html). Transaction handling is a planned feature but not yet implemented.

## Installation

(Provide instructions for installing the `xtdb-rs` driver in a Rust project.)

## Usage

`xtdb-rs` offers two primary functionalities:

1. **Viewing JSON Results**: Execute XTQL queries and view the results in a pretty-printed JSON format.
2. **Interactive Querying**: Read queries from input and send them to XTDB 2.x.

### Example: Viewing JSON Results

```rust
// Example code snippet to demonstrate JSON result viewing.

