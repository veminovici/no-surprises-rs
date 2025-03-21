//! No Surprises - A Rust library for musical theory and composition
//!
//! This crate provides a comprehensive set of tools for working with musical concepts
//! such as pitches, intervals, steps, and scales. It aims to make musical theory
//! calculations and manipulations intuitive and type-safe.
//!
//! # Features
//!
//! - Core musical types: `Pitch`, `Interval`, `Step`
//! - Conversion traits for musical elements
//! - Predefined constants for common musical values
//! - Scale implementations (major, natural minor, etc.)
//! - Generic const expressions for compile-time musical calculations
//!
//! # Examples
//!
//! ```rust
//! use no_surprises::prelude::*;
//! use no_surprises::scales::major_scale;
//!
//! // Create a pitch
//! let pitch = C4;
//!
//! // Work with intervals
//! let major_third = MAJOR_THIRD;
//!
//! // Create a scale
//! let scale = major_scale(C4);
//! ```
//!
//! # Modules
//!
//! - `core`: Contains fundamental musical types and traits
//! - `scales`: Provides implementations for various musical scales
//! - `prelude`: Re-exports commonly used types and constants

#![allow(incomplete_features)]
#![allow(unused)]
#![feature(generic_const_exprs)]

/// Core module containing fundamental musical types and traits
pub mod core;

/// Scales module providing implementations for various musical scales
pub mod scales;

/// The prelude module re-exports commonly used types and constants.
///
/// This module provides a convenient way to import the most commonly used
/// elements from the library. It includes:
///
/// - All core types (`Pitch`, `Interval`, `Step`)
/// - All conversion traits (`IntoIntervals`, `IntoSteps`, `IntoPitches`)
/// - All predefined constants
/// - Common scale types and functions
///
/// # Examples
///
/// ```rust
/// use no_surprises::prelude::*;
/// use no_surprises::scales::major_scale;
///
/// // Now you can use all the core types and constants directly
/// let pitch = C4;
/// let interval = MAJOR_THIRD;
/// let step = HALF;
///
/// // And work with scales
/// let scale = major_scale(C4);
/// ```
pub mod prelude {
    #[allow(unused_imports)]
    pub use crate::core::*;

    #[allow(unused_imports)]
    pub use crate::core::constants::*;
}

/// Re-export all items from the prelude for convenient access
#[allow(unused_imports)]
pub use prelude::*;
