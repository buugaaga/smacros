# smacros

[![Crates.io](https://img.shields.io/crates/v/s-macro)](https://crates.io/crates/smacros)
[![Docs.rs](https://docs.rs/s-macro/badge.svg)](https://docs.rs/smacros)

Minimal macro for easy `String` creation and concatenation in Rust.

## Usage

```rust
use smacros::s;

let s1 = s!("hello"); // From single value
let s2 = s!("a", "b", 42); // Concatenation
let empty = s!(); // Empty String
```

## Features

Converts any ToString value to String

Concatenates multiple values

Zero dependencies

Zero-cost abstraction

Add to Cargo.toml:

```toml
[dependencies]
s-macro = "0.1"
License: MIT OR Apache-2.0
```

This version:

1. Keeps all essential information
2. Has working badges
3. Shows basic usage
4. Lists key features
5. Includes installation instructions
6. Is short enough to copy-paste directly

For publishing:

1. Run `cargo publish --dry-run` first
2. Then `cargo publish`
3. Docs will auto-build on docs.rs
