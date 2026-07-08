# Sakka

[![Tag](https://img.shields.io/github/v/tag/tailflare/sakka)](https://github.com/tailflare/sakka/tags)
[![Crates.io Version](https://img.shields.io/crates/v/sakka)](https://crates.io/crates/sakka)
[![docs.rs](https://img.shields.io/docsrs/sakka)](https://docs.rs/sakka)
[![Main CI Build Status](https://img.shields.io/github/actions/workflow/status/tailflare/sakka/ci.yml?label=main%20build)](https://github.com/tailflare/sakka/actions/workflows/ci.yml)
[![License](https://img.shields.io/crates/l/sakka)](#license)

Sakka is a low-level framework for implementing binary file format parsers and writers in Rust.

It provides primitives for reading and writing binary data, including endian handling, alignment, collections, and custom codecs. Parsers and writers can be implemented manually for full control, or generated using derive macros for common data structures.

Sakka is designed for working with existing binary formats where precise control over byte layout is required, while also providing the foundation for creating new binary formats.

## License

The sakka project is licensed under either the Apache License, Version 2.0 or the MIT license, at your option.

See [LICENSE-APACHE](./LICENSE-APACHE) and [LICENSE-MIT](./LICENSE-MIT) for details.
