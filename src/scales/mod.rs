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
//! The module provides implementations for common scales through the heptatonic module,
//! which includes 7-note scales like major and natural minor. The module allows creating
//! custom scales by implementing the appropriate traits.

use crate::{Interval, Pitch, Step};
use std::marker::PhantomData;

/// Trait defining the quality of a scale (e.g., major, minor)
///
/// This trait provides the core properties of a scale:
/// - `STEPS_LENGTH`: Number of steps in the scale pattern
/// - `PITCHES_LENGTH`: Number of pitches (always steps + 1)
/// - `Pattern`: Type of the step pattern
/// - `STEPS_PATTERN`: The actual step pattern defining the scale
pub trait ScaleQuality {
    const STEPS_LENGTH: usize;
    const PITCHES_LENGTH: usize = Self::STEPS_LENGTH + 1;
    type Pattern: AsRef<[Step]>;
    const STEPS_PATTERN: Self::Pattern;
}

/// Generic scale type that can represent a scale in any form (intervals, steps, or pitches)
///
/// # Type Parameters
/// - `Q`: The scale quality (e.g., major, minor)
/// - `T`: The type of elements (Interval, Step, or Pitch)
/// - `N`: The number of elements in the scale
#[derive(Debug, PartialEq, Eq)]
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
    pub(crate) const fn new(items: [T; N]) -> Self {
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

use paste::paste;

/// Macro for defining a new scale type with all its representations and conversions
///
/// This macro generates:
/// - A quality struct (e.g., MajorQuality)
/// - Type aliases for different representations (steps, intervals, pitches)
/// - Implementations for converting between representations
/// - Helper functions for creating scales in different forms
///
/// # Arguments
/// - `$name`: The name of the scale (e.g., major)
/// - `$steps`: The step pattern defining the scale
macro_rules! define_scale {
     ($name:ident, $steps:expr) => {
        paste! {
            pub mod [<$name:lower>] {
                use crate::scales::*;
                use crate::prelude::*;

                #[derive(Debug, PartialEq, Eq)]
                pub struct [<$name Quality>];

                impl ScaleQuality for [<$name Quality>] {
                    const STEPS_LENGTH: usize = 7;
                    type Pattern = [Step; Self::STEPS_LENGTH];
                    const STEPS_PATTERN: Self::Pattern = $steps;
                }

                pub type [<$name ScaleSteps>] = Scale<[<$name Quality>], Step, { [<$name Quality>]::STEPS_LENGTH }>;
                pub type [<$name ScaleIntervals>] = Scale<[<$name Quality>], Interval, { [<$name Quality>]::STEPS_LENGTH }>;
                pub type [<$name ScalePitches>] = Scale<[<$name Quality>], Pitch, { [<$name Quality>]::PITCHES_LENGTH }>;

                impl [<$name ScaleSteps>] {
                    #[inline]
                    pub const fn steps(&self) -> &[Step; { [<$name Quality>]::STEPS_LENGTH }] {
                        &self.items
                    }

                    #[inline]
                    pub fn to_intervals(&self) -> [<$name ScaleIntervals>] {
                        [<$name ScaleIntervals>]::new(self.items.into_intervals())
                    }

                    #[inline]
                    pub fn to_pitches(&self) -> [<$name ScalePitches>] {
                        [<$name ScalePitches>]::new(self.items.into_pitches(C4))
                    }
                }

                impl [<$name ScaleIntervals>] {
                    #[inline]
                    pub const fn intervals(&self) -> &[Interval; { [<$name Quality>]::STEPS_LENGTH }] {
                        &self.items
                    }

                    #[inline]
                    pub fn to_steps(&self) -> [<$name ScaleSteps>] {
                        [<$name ScaleSteps>]::new(self.items.into_steps())
                    }

                    #[inline]
                    pub fn to_pitches(&self) -> [<$name ScalePitches>] {
                        [<$name ScalePitches>]::new(self.items.into_pitches(C4))
                    }
                }

                impl [<$name ScalePitches>] {
                    #[inline]
                    pub const fn pitches(&self) -> &[Pitch; { [<$name Quality>]::PITCHES_LENGTH }] {
                        &self.items
                    }

                    #[inline]
                    pub fn to_steps(&self) -> [<$name ScaleSteps>] {
                        [<$name ScaleSteps>]::new(self.items.into_steps())
                    }

                    #[inline]
                    pub fn to_intervals(&self) -> [<$name ScaleIntervals>] {
                        [<$name ScaleIntervals>]::new(self.items.into_intervals())
                    }
                }

                #[inline]
                pub fn [<$name:lower _scale_in_steps>]() -> [<$name ScaleSteps>] {
                    [<$name ScaleSteps>]::new([<$name Quality>]::STEPS_PATTERN)
                }

                #[inline]
                pub fn [<$name:lower _scale_in_intervals>]() -> [<$name ScaleIntervals>] {
                    [<$name ScaleIntervals>]::new([<$name Quality>]::STEPS_PATTERN.into_intervals())
                }

                #[inline]
                pub fn [<$name:lower _scale_in_pitches>](root: Pitch) -> [<$name ScalePitches>] {
                    [<$name ScalePitches>]::new([<$name Quality>]::STEPS_PATTERN.into_pitches(root))
                }

                #[inline]
                pub fn [<$name:lower _scale>](root: Pitch) -> [<$name ScalePitches>] {
                    [<$name ScalePitches>]::new([<$name Quality>]::STEPS_PATTERN.into_pitches(root))
                }

                pub(crate)mod constants {
                    use super::*;

                    pub const [<$name:upper _SCALE_STEPS>]: [Step; [<$name Quality>]::STEPS_LENGTH] = [<$name Quality>]::STEPS_PATTERN;
                }
            }
        }
    };
}

pub mod constants;

// Module for heptatonic scales (7-note scales)
mod heptatonic;

// Re-export heptatonic scales
pub use heptatonic::*;
