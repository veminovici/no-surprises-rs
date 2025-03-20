//! Scale module for handling musical scales
//!
//! This module provides functionality for working with musical scales in different representations:
//! - Intervals: The distances between scale degrees
//! - Steps: The smallest units of movement between scale degrees
//! - Pitches: The actual notes in the scale
//!
//! The module uses a generic `Scale` type that can represent a scale in any of these forms,
//! along with traits for converting between them.

use std::marker::PhantomData;

use crate::{Interval, Pitch, Step};

/// Marker trait for scale qualities (e.g., major, minor, pentatonic)
///
/// This trait is used to distinguish between different types of scales
/// while maintaining type safety. Implementations can add additional
/// functionality specific to certain scale qualities.
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

mod scale_intervals;
mod scale_pitches;
mod scale_steps;

pub use scale_intervals::*;
pub use scale_pitches::*;
pub use scale_steps::*;
