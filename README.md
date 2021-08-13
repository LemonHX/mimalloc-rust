# The Best and Highest-Leveled and Newest bingding for MiMalloc Ever Existed in Rust
> mimalloc 1.7.2 stable

![ci](https://github.com/LemonHX/mimalloc-rust/actions/workflows/rust.yml/badge.svg)

## Why create this

in repo `https://github.com/purpleprotocol/mimalloc_rust` i didn't see any data types and it neither has all the functionalities which mimalloc has provided, in other words it's garbage.

## Usage

first add to dependencies
```toml
[dependencies]
mimalloc-rust = "0.1"
```
then set the global allocator
```rust
use mimalloc_rust::*;

#[global_allocator]
static GLOBAL_MIMALLOC: GlobalMiMalloc = GlobalMiMalloc;
```