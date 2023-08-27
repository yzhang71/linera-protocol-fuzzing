# linera-protocol-fuzzing
[Fuzz testing](https://en.wikipedia.org/wiki/Fuzzing) is:

> An automated testing technique that involves providing invalid,
> unexpected, or random data as inputs to a program.

We use fuzz testing to automatically discover bugs in the linera-protocol.

The `fuzz/` directory contains the configuration and the fuzz tests
for linera-protocol. To generate and to run the fuzz tests, we use the
[`cargo-fuzz`] library.

## Installation

You may need to install the [`cargo-fuzz`] library to get the `cargo
fuzz` subcommand. Use

```sh
$ cargo install cargo-fuzz
```

`cargo-fuzz` is documented in the [Rust Fuzz
Book](https://rust-fuzz.github.io/book/cargo-fuzz.html).

## Running a fuzzer
```sh
$ cd /linera-protocol/linera-execution/fuzz
$ cargo fuzz run cached_contract_module
```

