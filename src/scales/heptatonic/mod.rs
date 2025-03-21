//! Heptatonic scales module
//!
//! This module provides implementations for 7-note scales, including:
//! - Major scale
//! - Natural minor scale
//!
//! Each scale is implemented with its own module containing the scale definition
//! and related constants.

pub mod major;
pub mod naturalminor;

pub use major::*;
pub use naturalminor::*;

pub mod constants {
    pub use super::major::constants::*;
    pub use super::naturalminor::constants::*;
}

pub use constants::*;
