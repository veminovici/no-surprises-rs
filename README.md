# no-surprises

[![Rust](https://github.com/veminovici/no-surprises-rs/actions/workflows/rust.yml/badge.svg)](https://github.com/veminovici/no-surprises-rs/actions/workflows/rust.yml)

A Rust library for working with musical scales, intervals, and pitches in a type-safe and predictable way.

## Overview

`no-surprises` provides a comprehensive set of tools for working with musical concepts in Rust. It offers type-safe abstractions for:
- Musical scales
- Intervals
- Pitches
- Steps

The library is designed to prevent common musical theory errors at compile time and provide a reliable foundation for music-related applications.

## Features

- Type-safe musical scale definitions
- Interval calculations and transformations
- Pitch manipulation and transposition
- Step-based scale navigation
- Compile-time validation of musical concepts

## Installation

Add this to your `Cargo.toml`:

```toml
[dependencies]
no-surprises = "0.1.0"
```

## Usage

```rust
use no_surprises::prelude::*;

// Create a scale
let scale = Scale::new(/* ... */);

// Work with intervals
let intervals = scale.into_intervals();

// Manipulate pitches
let pitches = scale.into_pitches(root_pitch);

// Navigate steps
let steps = scale.into_steps();
```

## Examples

The library includes several example applications that demonstrate its functionality:

```bash
# Run the constants example
cargo run --example 01_constants

# Run the operations example
cargo run --example 02_operations

# Run the slice operations example
cargo run --example 03_slice_operations
```

Each example demonstrates different aspects of the library:
- `01_constants`: Demonstrates the use of predefined musical constants
- `02_operations`: Shows various operations like adding steps to pitches and calculating distances
- `03_slice_operations`: Demonstrates working with slices of musical elements

## Contributing

Contributions are welcome! Please feel free to submit a Pull Request.

## License

This project is licensed under the MIT License - see the LICENSE file for details. 