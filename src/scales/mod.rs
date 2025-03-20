//! Scale module for handling musical scales
//!
//! This module provides functionality for working with musical scales in different representations:
//! - Intervals: The distances between scale degrees (e.g., MAJOR_SECOND, PERFECT_FOURTH)
//! - Steps: The smallest units of movement between scale degrees (e.g., WHOLE, HALF)
//! - Pitches: The actual notes in the scale (e.g., C4, D4, E4)
//!
//! The module uses a generic `Scale` type that can represent a scale in any of these forms,
//! along with traits for converting between them. For example, a major scale can be represented as:
//! - Intervals: [MAJOR_SECOND, MAJOR_THIRD, PERFECT_FOURTH, PERFECT_FIFTH, MAJOR_SIXTH, MAJOR_SEVENTH, PERFECT_OCTAVE]
//! - Steps: [WHOLE, WHOLE, HALF, WHOLE, WHOLE, WHOLE, HALF]
//! - Pitches: [C4, D4, E4, F4, G4, A4, B4, C5] (when starting from C4)
//!
//! The module provides implementations for common scales (like major) and allows creating
//! custom scales by implementing the appropriate traits.

use std::marker::PhantomData;

use crate::{Interval, Pitch, Step};

/// Marker trait for scale qualities (e.g., major, minor, pentatonic)
///
/// This trait is used to distinguish between different types of scales
/// while maintaining type safety. Implementations can add additional
/// functionality specific to certain scale qualities.
///
/// # Examples
///
/// ```
/// use no_surprises::scales::{ScaleQuality, MajorScaleQuality};
///
/// // MajorScaleQuality implements ScaleQuality
/// let _quality: &dyn ScaleQuality = &MajorScaleQuality;
/// ```
pub trait ScaleQuality {}

/// Trait for converting a scale into its interval representation
///
/// This trait allows converting a scale from any representation (steps or pitches)
/// into a sequence of intervals. The intervals represent the distances between
/// consecutive scale degrees.
///
/// # Type Parameters
///
/// * `Q` - The scale quality (e.g., major, minor)
///
/// # Examples
///
/// ```
/// use no_surprises::scales::{IntoScaleInIntervals, IntoScaleInSteps, ScaleInSteps, major_scale};
/// use no_surprises::prelude::*;
/// use no_surprises::scales::constants::*;
///
/// let steps = major_scale(C4).into_scale_in_steps::<7>();
/// let scale = steps.into_scale_in_intervals::<7>();
/// assert_eq!(scale.intervals(), &MAJOR_SCALE_INTERVALS);
/// ```
pub trait IntoScaleInIntervals<Q: ScaleQuality> {
    /// Converts the scale into a sequence of intervals
    ///
    /// # Type Parameters
    ///
    /// * `M` - The number of intervals in the resulting scale
    ///
    /// # Returns
    ///
    /// A scale containing the intervals between scale degrees
    fn into_scale_in_intervals<const M: usize>(self) -> Scale<Q, Interval, M>;
}

/// Trait for converting a scale into its step representation
///
/// This trait allows converting a scale from any representation (intervals or pitches)
/// into a sequence of steps. The steps represent the smallest units of movement
/// between consecutive scale degrees.
///
/// # Type Parameters
///
/// * `Q` - The scale quality (e.g., major, minor)
///
/// # Examples
///
/// ```
/// use no_surprises::scales::{IntoScaleInSteps, ScaleInIntervals, IntoScaleInIntervals, major_scale};
/// use no_surprises::prelude::*;
/// use no_surprises::scales::constants::*;
///
/// let scale = major_scale(C4).into_scale_in_intervals::<7>();
/// let scale = scale.into_scale_in_steps::<7>();
///
/// assert_eq!(scale.steps(), &MAJOR_SCALE_STEPS);
/// ```
pub trait IntoScaleInSteps<Q: ScaleQuality> {
    /// Converts the scale into a sequence of steps
    ///
    /// # Type Parameters
    ///
    /// * `M` - The number of steps in the resulting scale
    ///
    /// # Returns
    ///
    /// A scale containing the steps between scale degrees
    fn into_scale_in_steps<const M: usize>(self) -> Scale<Q, Step, M>;
}

/// Trait for converting a scale into its pitch representation
///
/// This trait allows converting a scale from any representation (intervals or steps)
/// into a sequence of pitches. The pitches represent the actual notes in the scale,
/// starting from a given root pitch.
///
/// # Type Parameters
///
/// * `Q` - The scale quality (e.g., major, minor)
///
/// # Examples
///
/// ```
/// use no_surprises::scales::{IntoScaleInPitches, IntoScaleInSteps, ScaleInSteps, major_scale};
/// use no_surprises::prelude::*;
/// use no_surprises::scales::constants::*;
///
/// let steps = major_scale(C4).into_scale_in_steps::<7>();
/// let pitches = steps.into_scale_in_pitches::<8>(C4);
/// assert_eq!(pitches.items(), &[C4, D4, E4, F4, G4, A4, B4, C5]);
/// ```
pub trait IntoScaleInPitches<Q: ScaleQuality> {
    /// Converts the scale into a sequence of pitches
    ///
    /// # Type Parameters
    ///
    /// * `M` - The number of pitches in the resulting scale
    ///
    /// # Arguments
    ///
    /// * `root` - The root pitch to start the scale from
    ///
    /// # Returns
    ///
    /// A scale containing the pitches, starting from the root pitch
    fn into_scale_in_pitches<const M: usize>(self, root: Pitch) -> Scale<Q, Pitch, M>;
}

/// A generic scale type that can represent a scale in any form
///
/// This struct provides a unified way to work with scales in different
/// representations (intervals, steps, or pitches) while maintaining type
/// safety through the `ScaleQuality` marker trait.
///
/// # Type Parameters
///
/// * `Q` - The scale quality (e.g., major, minor)
/// * `T` - The type of elements in the scale (Interval, Step, or Pitch)
/// * `N` - The number of elements in the scale
///
/// # Examples
///
/// ```
/// use no_surprises::scales::{Scale, MajorScaleQuality, major_scale, IntoScaleInSteps, IntoScaleInIntervals};
/// use no_surprises::scales::constants::*;
/// use no_surprises::prelude::*;
///
/// // Create a scale with intervals
/// let scale = major_scale(C4).into_scale_in_intervals();
/// assert_eq!(scale.intervals(), &MAJOR_SCALE_INTERVALS);
///
/// // Create a scale with steps
/// let scale = major_scale(C4).into_scale_in_steps();
/// assert_eq!(scale.steps(), &MAJOR_SCALE_STEPS);
///
/// // Create a scale with pitches
/// let scale = major_scale(C4);
/// assert_eq!(scale.pitches(), &[C4, D4, E4, F4, G4, A4, B4, C5]);
/// ```
#[derive(Debug)]
pub struct Scale<Q: ScaleQuality, T, const N: usize> {
    /// Phantom data to maintain the scale quality type parameter
    quality: PhantomData<Q>,
    /// The elements of the scale
    items: [T; N],
}

impl<Q: ScaleQuality, T, const N: usize> Scale<Q, T, N> {
    /// Creates a new scale with the given elements
    ///
    /// # Arguments
    ///
    /// * `items` - The elements of the scale
    ///
    /// # Returns
    ///
    /// A new scale containing the given elements
    #[inline]
    pub const fn new(items: [T; N]) -> Self {
        Self {
            items,
            quality: PhantomData,
        }
    }

    /// Returns a reference to the elements of the scale
    ///
    /// # Returns
    ///
    /// A reference to the array of elements
    #[inline]
    pub const fn items(&self) -> &[T; N] {
        &self.items
    }
}

pub mod constants;

mod heptatonic;

mod scale_intervals;
mod scale_pitches;
mod scale_steps;

pub use heptatonic::*;
pub use scale_intervals::*;
pub use scale_pitches::*;
pub use scale_steps::*;
