# no-surprises

[![Rust](https://github.com/veminovici/no-surprises-rs/actions/workflows/rust.yml/badge.svg)](https://github.com/veminovici/no-surprises-rs/actions/workflows/rust.yml)

A Rust library for working with musical scales, intervals, and pitches in a type-safe and predictable way.

## Overview

`no-surprises` provides a comprehensive set of tools for working with musical concepts in Rust. It offers type-safe abstractions for:
- Musical scales (including built-in support for common scales like major)
- Intervals
- Pitches
- Steps

The library is designed to prevent common musical theory errors at compile time and provide a reliable foundation for music-related applications.

## Features

- Type-safe musical scale definitions
- Multiple scale representations (intervals, steps, pitches)
- Built-in support for common scales (major, etc.)
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
use no_surprises::scales::major_scale;

// Create a major scale starting from C4
let scale = major_scale(C4);
assert_eq!(scale.pitches(), &[C4, D4, E4, F4, G4, A4, B4, C5]);

// Convert between scale representations
let steps = scale.into_scale_in_steps();
assert_eq!(steps.steps(), &[WHOLE, WHOLE, HALF, WHOLE, WHOLE, WHOLE, HALF]);

let intervals = scale.into_scale_in_intervals();
assert_eq!(intervals.intervals(), &[
    MAJOR_SECOND,
    MAJOR_THIRD,
    PERFECT_FOURTH,
    PERFECT_FIFTH,
    MAJOR_SIXTH,
    MAJOR_SEVENTH,
    PERFECT_OCTAVE
]);

// Create a scale with custom steps
let custom_scale = ScaleInSteps::<MyScaleQuality, 7>::new([
    WHOLE, WHOLE, HALF, WHOLE, WHOLE, WHOLE, HALF
]);

// Get the steps in the scale
let steps = custom_scale.steps();

// Convert into a scale on intervals
let scale = custom_scale.into_scale_in_intervals::<7>();

// Get the intervals in the scale
let intervals = scale.intervals();

// Convert into a scale on pitches
let scale = scale.into_scale_in_pitches::<8>(C4);

// Get the pitches in the scale
let pitches = scale.pitches();
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

# Run the scale operations example
cargo run --example 04_basic_scales

# Run the major scale example
cargo run --example 05_major_scale
```

Each example demonstrates different aspects of the library:
- `01_constants`: Demonstrates the use of predefined musical constants
- `02_operations`: Shows various operations like adding steps to pitches and calculating distances
- `03_slice_operations`: Demonstrates working with slices of musical elements
- `04_basic_scales`: Shows various operations like transforming from scales based on steps to ones based on intervals
- `05_major_scale`: Demonstrates working with the built-in major scale implementation

## Contributing

Contributions are welcome! Please feel free to submit a Pull Request.

## License

This project is licensed under the MIT License - see the LICENSE file for details. 