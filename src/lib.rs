//! No Surprises - A Rust library for music theory and composition
//!
//! This library provides a set of tools for working with musical concepts in Rust.
//! It focuses on providing a simple, intuitive API for common music theory operations
//! while maintaining type safety and performance.
//!
//! # Features
//!
//! - **Pitch Handling**: Work with MIDI note numbers and musical pitches
//! - **Interval Calculations**: Calculate and manipulate musical intervals
//! - **Step Operations**: Work with the smallest units of pitch movement
//! - **Type Conversions**: Convert between pitches, intervals, and steps
//! - **Predefined Constants**: Common musical elements like notes and intervals
//!
//! # Examples
//!
//! ```rust
//! use no_surprises::prelude::*;
//!
//! // Create a pitch and add a step
//! let c4 = C4; // Middle C
//! let d4 = c4 + WHOLE;
//!
//! // Work with intervals
//! let interval = MAJOR_THIRD;
//! let step = Step::from(interval);
//!
//! // Convert between types
//! let pitches = [C4, D4, E4];
//! let intervals: [Interval; 2] = pitches.into_intervals();
//! ```
//!
//! # Modules
//!
//! - `core`: The main module containing fundamental types and traits
//!   - `Pitch`: Represents musical notes
//!   - `Interval`: Represents distances between pitches
//!   - `Step`: Represents the smallest pitch movement
//!   - Various conversion traits for working with sequences
//!
//! # Constants
//!
//! The library provides predefined constants for common musical elements:
//!
//! ```rust
//! use no_surprises::prelude::*;
//!
//! // Pitch constants
//! let middle_c = C4;      // MIDI note 60
//! let d4 = D4;           // MIDI note 62
//! let e4 = E4;           // MIDI note 64
//!
//! // Interval constants
//! let unison = PERFECT_UNISON;    // 0 semitones
//! let major_second = MAJOR_SECOND; // 2 semitones
//! let perfect_fifth = PERFECT_FIFTH; // 7 semitones
//!
//! // Step constants
//! let half_step = HALF;   // 1 semitone
//! let whole_step = WHOLE; // 2 semitones
//! ```
//!
//! # Safety and Performance
//!
//! The library is designed with the following principles:
//! - Type safety: All operations are checked at compile time
//! - Zero-cost abstractions: High-level operations compile to efficient code
//! - No panics: Operations are designed to be safe and predictable
//! - Minimal dependencies: Core functionality has no external dependencies
//!
//! # License
//!
//! This project is licensed under the MIT License - see the LICENSE file for details.

pub mod core;

/// The prelude module re-exports commonly used types and constants.
///
/// This module provides a convenient way to import the most commonly used
/// elements from the library. It includes:
///
/// - All core types (`Pitch`, `Interval`, `Step`)
/// - All conversion traits (`IntoIntervals`, `IntoSteps`, `IntoPitches`)
/// - All predefined constants
///
/// # Examples
///
/// ```rust
/// use no_surprises::prelude::*;
///
/// // Now you can use all the core types and constants directly
/// let pitch = C4;
/// let interval = MAJOR_THIRD;
/// let step = HALF;
/// ```
pub mod prelude {
    #[allow(unused_imports)]
    pub use crate::core::*;

    #[allow(unused_imports)]
    pub use crate::core::constants::*;
}

#[allow(unused_imports)]
pub use prelude::*;
