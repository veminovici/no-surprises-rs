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

/// Trait for validating scale lengths
pub trait ValidateScaleLength {
    const MIN_LENGTH: usize;
    const MAX_LENGTH: usize;

    fn validate_length(length: usize) -> bool {
        length >= Self::MIN_LENGTH && length <= Self::MAX_LENGTH
    }
}

/// Trait for scale quality with additional validation
pub trait ScaleQuality: Sized {
    const STEPS_LENGTH: usize;
    const PITCHES_LENGTH: usize = Self::STEPS_LENGTH + 1;
    type Pattern: AsRef<[Step]>;
    const STEPS_PATTERN: Self::Pattern;

    // Add validation for scale pattern
    fn validate_pattern(pattern: &[Step]) -> bool {
        pattern.len() == Self::STEPS_LENGTH
    }
}

/// Trait for validating scale steps
pub trait ValidateScaleSteps {
    fn validate_steps(steps: &[Step]) -> bool {
        // Ensure steps are positive
        steps.iter().all(|&s| s.semitones() > 0)
    }
}

/// Trait for validating scale intervals
pub trait ValidateScaleIntervals {
    fn validate_intervals(intervals: &[Interval]) -> bool {
        // Ensure intervals are in ascending order
        intervals.windows(2).all(|w| w[0] <= w[1])
    }
}

/// Trait for validating scale pitches
pub trait ValidateScalePitches {
    fn validate_pitches(pitches: &[Pitch], length: usize) -> bool {
        // check the length of the pitches
        if pitches.len() != length {
            return false;
        }

        // Ensure pitches are in ascending order
        pitches.windows(2).all(|w| w[0] <= w[1])
    }
}

/// Generic scale type with additional validation
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Scale<Q: ScaleQuality + ValidateScaleLength + ValidateScaleSteps, T, const N: usize> {
    items: [T; N],
    _quality: PhantomData<Q>,
}

impl<Q: ScaleQuality + ValidateScaleLength + ValidateScaleSteps, T, const N: usize> Scale<Q, T, N>
where
    T: std::fmt::Debug,
{
    /// Creates a new scale with validation
    pub fn new(items: [T; N]) -> Self {
        assert!(
            Q::validate_pattern(Q::STEPS_PATTERN.as_ref()),
            "Invalid scale pattern"
        );

        Self {
            items,
            _quality: PhantomData,
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

/// Macro for defining the quality struct and its implementation
macro_rules! define_scale_quality {
    ($name:ident, $steps:expr) => {
        paste! {
            #[derive(Debug, PartialEq, Eq)]
            pub struct [<$name Quality>];

            impl ScaleQuality for [<$name Quality>] {
                const STEPS_LENGTH: usize = 7;
                type Pattern = [Step; Self::STEPS_LENGTH];
                const STEPS_PATTERN: Self::Pattern = $steps;
            }

            impl ValidateScaleLength for [<$name Quality>] {
                const MIN_LENGTH: usize = 7;
                const MAX_LENGTH: usize = 7;
            }

            impl ValidateScaleSteps for [<$name Quality>] {
            }

            impl ValidateScaleIntervals for [<$name Quality>] {
            }

            impl ValidateScalePitches for [<$name Quality>] {
            }
        }
    };
}

/// Macro for defining type aliases for different scale representations
macro_rules! define_scale_types {
    ($name:ident) => {
        paste! {
            pub type [<$name ScaleSteps>] = Scale<[<$name Quality>], Step, { [<$name Quality>]::STEPS_LENGTH }>;
            pub type [<$name ScaleIntervals>] = Scale<[<$name Quality>], Interval, { [<$name Quality>]::STEPS_LENGTH }>;
            pub type [<$name ScalePitches>] = Scale<[<$name Quality>], Pitch, { [<$name Quality>]::PITCHES_LENGTH }>;
        }
    };
}

/// Macro for implementing scale step operations
macro_rules! impl_scale_steps {
    ($name:ident) => {
        paste! {
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
        }
    };
}

/// Macro for implementing scale interval operations
macro_rules! impl_scale_intervals {
    ($name:ident) => {
        paste! {
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
        }
    };
}

/// Macro for implementing scale pitch operations
macro_rules! impl_scale_pitches {
    ($name:ident) => {
        paste! {
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
        }
    };
}

/// Macro for defining scale helper functions
macro_rules! define_scale_functions {
    ($name:ident) => {
        paste! {
            #[inline]
            pub fn [<$name:lower _scale_in_steps>]() -> [<$name ScaleSteps>] {

                let steps = [<$name Quality>]::STEPS_PATTERN;

                debug_assert!(
                    [<$name Quality>]::validate_steps(&steps),
                    "Invalid steps. steps: {:?}",
                    steps);

                [<$name ScaleSteps>]::new(steps)
            }

            #[inline]
            pub fn [<$name:lower _scale_in_intervals>]() -> [<$name ScaleIntervals>] {
                let intervals = [<$name Quality>]::STEPS_PATTERN.into_intervals();

                debug_assert!(
                    [<$name Quality>]::validate_intervals(&intervals),
                    "Invalid intervals. intervals: {:?}",
                    intervals);

                [<$name ScaleIntervals>]::new(intervals)
            }

            #[inline]
            pub fn [<$name:lower _scale_in_pitches>](root: Pitch) -> [<$name ScalePitches>] {
                let pitches = [<$name Quality>]::STEPS_PATTERN.into_pitches(root);
                let length = [<$name Quality>]::PITCHES_LENGTH;

                debug_assert!(
                    [<$name Quality>]::validate_pitches(&pitches, length),
                    "Invalid pitches. length: {}, pitches: {:?}",
                    length,
                    pitches);

                [<$name ScalePitches>]::new(pitches)
            }

            #[inline]
            pub fn [<$name:lower _scale>](root: Pitch) -> [<$name ScalePitches>] {
                let pitches = [<$name Quality>]::STEPS_PATTERN.into_pitches(root);
                let length = [<$name Quality>]::PITCHES_LENGTH;

                debug_assert!(
                    [<$name Quality>]::validate_pitches(&pitches, length),
                    "Invalid pitches. length: {}, pitches: {:?}",
                    length,
                    pitches);

                [<$name ScalePitches>]::new(pitches)
            }
        }
    };
}

/// Macro for defining scale constants
macro_rules! define_scale_constants {
    ($name:ident) => {
        paste! {
            pub(crate) mod constants {
                use super::*;

                pub const [<$name:upper _SCALE_STEPS>]: [Step; [<$name Quality>]::STEPS_LENGTH] = [<$name Quality>]::STEPS_PATTERN;
            }
        }
    };
}

/// Main macro for defining a new scale type with all its representations and conversions
///
/// This macro combines all the smaller macros to generate a complete scale implementation:
/// - Scale quality and implementation
/// - Type aliases for different representations
/// - Conversion implementations
/// - Helper functions
/// - Constants
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

                define_scale_quality!($name, $steps);
                define_scale_types!($name);
                impl_scale_steps!($name);
                impl_scale_intervals!($name);
                impl_scale_pitches!($name);
                define_scale_functions!($name);
                define_scale_constants!($name);
            }
        }
    };
}

pub mod constants;

// Module for heptatonic scales (7-note scales)
mod heptatonic;

// Re-export heptatonic scales
pub use heptatonic::*;
