//! Core module for the no-surprises music theory library
//!
//! This module provides the fundamental types and traits for working with musical concepts:
//! - `Pitch`: Represents a specific musical note in the MIDI system
//! - `Interval`: Represents the distance between two pitches
//! - `Step`: Represents the smallest unit of pitch movement
//!
//! The module also provides traits for converting between these types:
//! - `IntoIntervals`: Converts a sequence of elements into intervals
//! - `IntoSteps`: Converts a sequence of elements into steps
//! - `IntoPitches`: Converts a sequence of elements into pitches
//!
//! # Examples
//!
//! ```rust
//! use no_surprises::core::*;
//!
//! // Create a pitch and add a step to it
//! let pitch = Pitch::new(60);
//! let new_pitch = pitch + Step::new(2);
//!
//! // Convert a sequence of pitches to intervals
//! let pitches = [Pitch::new(60), Pitch::new(62), Pitch::new(65)];
//! let intervals: [Interval; 2] = pitches.into_intervals();
//! ```
//!
//! # Constants
//!
//! The module provides predefined constants for common musical elements:
//! - Pitches: `C4`, `D4`, `E4`, etc.
//! - Intervals: `PERFECT_UNISON`, `MAJOR_SECOND`, etc.
//! - Steps: `HALF`, `WHOLE`, etc.

pub(crate) const SEMITONES_IN_OCTAVE: u8 = 12;

pub mod constants;

mod interval;
mod pitch;
mod step;

pub use interval::*;
pub use pitch::*;
pub use step::*;

/// Trait for converting a sequence of elements into intervals.
///
/// This trait is implemented for arrays of pitches and steps, allowing them to be
/// converted into sequences of intervals. The conversion process depends on the
/// type of element being converted:
///
/// - For pitches: intervals are calculated relative to the first pitch
/// - For steps: intervals are calculated by accumulating the steps
///
/// # Examples
///
/// ```rust
/// use no_surprises::core::*;
///
/// // Convert pitches to intervals
/// let pitches = [Pitch::new(60), Pitch::new(62), Pitch::new(65)];
/// let intervals: [Interval; 2] = pitches.into_intervals();
///
/// // Convert steps to intervals
/// let steps = [Step::new(2), Step::new(3), Step::new(4)];
/// let intervals: [Interval; 3] = steps.into_intervals();
/// ```
pub trait IntoIntervals {
    /// Converts the sequence into an array of intervals.
    ///
    /// # Arguments
    ///
    /// * `self` - The sequence to convert
    ///
    /// # Returns
    ///
    /// An array of intervals
    fn into_intervals<const M: usize>(self) -> [Interval; M];
}

/// Trait for converting a sequence of elements into steps.
///
/// This trait is implemented for arrays of pitches and intervals, allowing them to be
/// converted into sequences of steps. The conversion process depends on the
/// type of element being converted:
///
/// - For pitches: steps are calculated as the difference between consecutive pitches
/// - For intervals: steps are calculated as the difference between consecutive intervals
///
/// # Examples
///
/// ```rust
/// use no_surprises::core::*;
///
/// // Convert pitches to steps
/// let pitches = [Pitch::new(60), Pitch::new(62), Pitch::new(65)];
/// let steps: [Step; 2] = pitches.into_steps();
///
/// // Convert intervals to steps
/// let intervals = [Interval::new(2), Interval::new(5), Interval::new(9)];
/// let steps: [Step; 3] = intervals.into_steps();
/// ```
pub trait IntoSteps {
    /// Converts the sequence into an array of steps.
    ///
    /// # Arguments
    ///
    /// * `self` - The sequence to convert
    ///
    /// # Returns
    ///
    /// An array of steps
    fn into_steps<const M: usize>(self) -> [Step; M];
}

/// Trait for converting a sequence of elements into pitches.
///
/// This trait is implemented for arrays of intervals and steps, allowing them to be
/// converted into sequences of pitches. The conversion process requires a root pitch
/// to start from, and then applies the intervals or steps sequentially.
///
/// # Examples
///
/// ```rust
/// use no_surprises::core::*;
///
/// // Convert intervals to pitches
/// let intervals = [Interval::new(2), Interval::new(5), Interval::new(9)];
/// let pitches: [Pitch; 4] = intervals.into_pitches(Pitch::new(60));
///
/// // Convert steps to pitches
/// let steps = [Step::new(2), Step::new(3), Step::new(4)];
/// let pitches: [Pitch; 4] = steps.into_pitches(Pitch::new(60));
/// ```
pub trait IntoPitches {
    /// Converts the sequence into an array of pitches.
    ///
    /// # Arguments
    ///
    /// * `self` - The sequence to convert
    /// * `root` - The root pitch to start from
    ///
    /// # Returns
    ///
    /// An array of pitches starting from the root pitch
    fn into_pitches<const M: usize>(self, root: Pitch) -> [Pitch; M];
}
