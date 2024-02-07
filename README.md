# Rust Driver for XTDB 2.x

`xtdb-rs` is a Rust driver designed for interacting with XTDB 2.x. This project focuses on providing a seamless way to execute XTQL queries within Rust code, ensuring complete parsing of XTQL expressions with meaningful error messages.

## Features

- **Execute Queries**: Directly execute XTQL queries and receive responses in JSON format.
- **Transaction Handling**: Ability to handle transactions (currently under development).
- **XTQL Integration**: Embed XTQL in your Rust code for efficient query operations.
- **Error Handling**: Provides comprehensive error handling with meaningful error descriptions.

## Current Limitations

- **Incomplete**: The project is currently incomplete and lacks comprehensive tests.
- **Query Focused**: Currently, the driver supports only query operations. Transaction handling is a planned feature.

## Installation

1. **Install Rust**: Follow the instructions to install Rust on your system from [The Rust Programming Language website](https://www.rust-lang.org/learn/get-started).
2. **Install XTDB 2.x**: To run XTDB locally, refer to the guide on [Running XTDB 2.x Locally](https://docs.xtdb.com/operations-guide/).
3. **Build the Project**: Clone the repository and build the project using Cargo.
    ```bash
    git clone https://github.com/jsulmont/xtdb-rs
    cd xtdb-rs
    cargo build
    ```

## Usage

`xtdb-rs` offers functionalities to execute XTQL queries and view the results in JSON format or through interactive querying.

### Examples

#### Viewing JSON Results

Run an XTQL query and view the results in JSON format:

```bash
cat q-tpch/q11.edn | ./xtql_to_json | jq

