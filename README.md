# Rust Driver for XTDB 2.x

`xtdb-rs` is a Rust driver designed for interacting with XTDB 2.x. This project focuses on providing a seamless way to execute XTQL queries within Rust code, ensuring complete parsing of XTQL expressions with meaningful error messages.

## Background

[XTDB 2.x](https://xtdb.com/v2) is primarily a server-side technology developped by [Juxt](https://www.juxt.pro) that can be interacted with over various protocols:

- **HTTP (Transit+JSON)**: Traditional HTTP protocol using Transit and JSON for data serialization.
- **HTTP (JSON-LD)**: A JSON-based format to serialize Linked Data, leveraging the HTTP protocol.
- **pg_wire**: Implements the PostgreSQL wire protocol. For more details, refer to [PostgreSQL Wire Protocol documentation](https://www.postgresql.org/docs/current/protocol.html).
- **Arrow Flight SQL**: A high-performance protocol for large-scale data transfer. More information can be found at [Arrow Flight SQL format](https://arrow.apache.org/docs/format/FlightSql.html).

Note that the implementations for pg_wire and Arrow Flight SQL are currently a work in progress.

XTDB 2.x supports two main query languages:

- **SQL**: The standard language for managing and manipulating databases.
- **XTQL**: A powerful query language specifically designed for XTDB. More information on XTQL can be found in the [XTDB XTQL Reference](https://docs.xtdb.com/reference/main.html).

An example of a Common Lisp client using HTTP/Transit+JSON with XTDB is [xtdb-cl](https://github.com/jsulmont/xtdb-cl).


## Features

- **Execute Queries**: Directly execute XTQL queries and receive responses in JSON format.
- **Transaction Handling**: Ability to handle transactions (currently under development).
- **XTQL Integration**: Embed XTQL in your Rust code for efficient query operations.
- **Error Handling**: Provides comprehensive error handling with meaningful error descriptions.

## Current Limitations

- **Incomplete**: The project is currently incomplete and lacks comprehensive tests.
- **Query Focused**: Currently, the driver supports only query operations. Transaction handling is a planned feature.
- **

## Installation

1. **Install Rust**: Follow the instructions to install Rust on your system from [The Rust Programming Language website](https://www.rust-lang.org/learn/get-started).
2. **Install XTDB 2.x**: To run XTDB locally, refer to the guide on [Running XTDB 2.x Locally](https://docs.xtdb.com/intro/getting-started).
3. **Build the Project**: Clone the repository and build the project using Cargo.

    ```bash
    git clone https://github.com/jsulmont/xtdb-rs
    cd xtdb-rs
    cargo build
    ```

## Usage

`xtdb-rs` offers functionalities to execute XTQL queries and view the results in JSON format or through interactive querying.

### Examples

#### Build the examples
```bash
cargo build --examples
```

#### Viewing JSON Results

Run an XTQL query and view the results in JSON format:

```bash
cat q-tpch/q11.edn | ./xtql_to_json | jq
```

## License

`xtdb-rs` is licensed under the MIT License - see the [LICENSE](LICENSE) file for details.

