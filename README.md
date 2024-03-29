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

## About XTQL

XTQL is a domain-specific language (DSL) inspired by [Datalog](https://en.wikipedia.org/wiki/Datalog), a declarative logic programming language. XTQL is primarily used with XTDB, offering a unique approach to crafting database queries.

### TL;DR
Consider the following (valid albeit meaningless) XTQL query:

```clojure
(rel
 {:x (+ 1.77 (- 23 (if (= 0 x) 1 x))),
  :y (case
      (:order/status order)
      "pending"
      (calculate-pending-total)
      "completed"
      (calculate-completed-total)
      (aggregate-default-action)),
  :z #inst "1970-01-01T00:00:00.000-00:00"}
 [x y])
```
and try to format and edit the following its JSON counterpart:

```json
{"bind":[{"x":{"xt:lvar":"x"}},{"y":{"xt:lvar":"y"}}],"rel":{"x":{"args":[1.77,{"args":[23,{"args":[{"args":[0,{"xt:lvar":"x"}],"xt:call":"="},1,{"xt:lvar":"x"}],"xt:call":"if"}],"xt:call":"-"}],"xt:call":"+"},"y":{"args":[{"args":[{"xt:lvar":"order"}],"xt:call":"order/status"},"pending",{"args":[],"xt:call":"calculate-pending-total"},"completed",{"args":[],"xt:call":"calculate-completed-total"},{"args":[],"xt:call":"aggregate-default-action"}],"xt:call":"case"},"z":{"@type":"xt:instant","@value":"1970-01-01"}}}
```


### Key Features of XTQL

- **S-Expression Based**: XTQL leverages s-expressions, the simple and powerful syntactic structure used in Lisp. This design choice makes XTQL not only elegant but also concise, providing a clear and efficient way to represent queries.

- **Elegance and Conciseness**: One of XTQL's standout features is its elegant and concise syntax. It allows users to express complex queries in a more readable and maintainable form, reducing the cognitive load typically associated with verbose query languages.

- **Ease of Reasoning**: For those familiar with Lisp or similar languages, XTQL feels natural and intuitive. Its structure and syntax make it easy to reason about, offering a straightforward way to understand and manipulate complex queries.

- **Inspired by Datalog**: By drawing inspiration from Datalog, XTQL benefits from the strengths of logic programming. This makes it well-suited for queries that involve complex relationships and hierarchical data, where traditional SQL might fall short.

XTQL's design not only facilitates effective database interactions but also aligns with the thought processes of many developers, particularly those accustomed to Lisp-like languages. This alignment makes it an appealing choice for many, offering a blend of simplicity, power, and expressiveness.


## Development Status: Work in Progress

### Warning: Evolving Project

This project is currently in an active development phase and is considered a work in progress. Users and contributors should be aware of the following aspects:

- **Grammar**: The [grammar](src/xtql/pest/xtql.pest) for parsing XTQL is based on fragments of EBNF found in the XTQL documentation. As such, it is subject to change and evolution. 
- **`parse_value` Function**: The `parse_value` function is a key part of our parsing logic, and there's a possibility that it may be optimized or refactored in the future. View the function in the repository [here](src/xtql/parse.rs#L70).
- **Query Parsing Approach**: During development, often the JSON result of calling `parse-query` on some EDN was examined for better understanding and implementation. This process can be reviewed in the XTDB source code [here](https://github.com/xtdb/xtdb/blob/2.x/api/src/main/clojure/xtdb/xtql/edn.clj#L19).

Contributors and users should expect updates and changes as the project progresses. Feedback and contributions are welcome to improve and evolve the project further.


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

#### Executing a query

Assuming you have loaded a TPCH dataset (_e.g.,_ scale 0.05), then you can execute a query as following:

```bash
cat q-tpch/q2.edn| ./xtdb_query
```
, which will produce an output like (some elements ommited):
```json
[
  {
    "n": "nationkey_7",
    "n-name": "GERMANY",
    "p": "partkey_1634",
    "p-mfgr": "Manufacturer#2",
    "p-type": "ECONOMY PLATED BRASS",
    "ps-supplycost": 372.29,
    "r": "regionkey_3",
    "s": "suppkey_135",
    "s-acctbal": 9767.99,
    "s-address": "F4Uy ZQNU6ESTmO3mrL,mI",
    "s-comment": "courts wake slyly instructions. furiously silent requests cajol",
    "s-name": "Supplier#000000135",
    "s-phone": "17-290-812-8855"
  },
  {
    "n": "nationkey_7",
    "n-name": "GERMANY",
    "p": "partkey_2156",
    "p-mfgr": "Manufacturer#5",
    "p-type": "LARGE ANODIZED BRASS",
    "ps-supplycost": 809.04,
    "r": "regionkey_3",
    "s": "suppkey_44",
    "s-acctbal": 9759.38,
    "s-address": "kERxlLDnlIZJdN66zAPHklyL",
    "s-comment": "x. carefully quiet account",
    "s-name": "Supplier#000000044",
    "s-phone": "17-713-930-5667"
  }
]
```





## License

`xtdb-rs` is licensed under the MIT License - see the [LICENSE](LICENSE) file for details.

