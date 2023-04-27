# The Best and Highest-Leveled and Newest binding for MiMalloc Ever Existed in Rust
> mimalloc 1.7.9 stable

![ci](https://github.com/LemonHX/mimalloc-rust/actions/workflows/rust.yml/badge.svg)
[![doc.rs](https://docs.rs/mimalloc-rust/badge.svg)](https://docs.rs/mimalloc-rust)
[![crates.io](https://img.shields.io/crates/v/mimalloc-rust.svg)](https://crates.io/crates/mimalloc-rust)

## Why create this

in repo `https://github.com/purpleprotocol/mimalloc_rust` i didn't see any data types and it neither has all the functionalities which mimalloc has provided, in other words it's garbage.

## Usage

first add to dependencies
```toml
[dependencies]
mimalloc-rust = "0.2.1"
```
then set the global allocator
```rust
use mimalloc_rust::*;

#[global_allocator]
static GLOBAL_MIMALLOC: GlobalMiMalloc = GlobalMiMalloc;
```
