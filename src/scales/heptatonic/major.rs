//! Module for the major scale, one of the most fundamental scales in Western music.
//!
//! The major scale follows the pattern: whole, whole, half, whole, whole, whole, half
//! This creates a bright, happy sound characteristic of major scales.
//!
//! The scale can be represented in three ways:
//! 1. As intervals from the root note
//! 2. As steps between consecutive notes
//! 3. As actual pitches starting from a root note
//!
//! This module provides implementations for all three representations.

use constants::MAJOR_SCALE_IN_INTERVALS;

use crate::Pitch;
use crate::scales::{
    ScaleInIntervals, ScaleInPitches, ScaleInSteps, ScaleQuality, ToScaleInPitches,
};

/// Quality marker for major scales.
///
/// This type is used to distinguish major scales from other scale types
/// in the type system.
pub struct MajorScaleQuality;
impl ScaleQuality for MajorScaleQuality {
    /// The type alias for a major scale represented as steps
    type Steps = ScaleInSteps<Self, { Self::STEPS }>;

    /// The type alias for a major scale represented as intervals
    type Intervals = ScaleInIntervals<Self, { Self::INTERVALS }>;

    /// The type alias for a major scale represented as pitches
    type Pitches = ScaleInPitches<Self, { Self::PITCHES }>;

    /// The number of steps in a major scale (excluding the octave)
    const STEPS: usize = 7;

    /// The number of intervals in a major scale (excluding the octave)
    const INTERVALS: usize = Self::STEPS;

    /// The number of pitches in a major scale (including the octave)
    const PITCHES: usize = Self::STEPS + 1;
}

/// Creates a major scale starting from the given root pitch.
///
/// # Arguments
///
/// * `root` - The root pitch to start the scale from
///
/// # Returns
///
/// A major scale represented as an array of pitches, including the octave
///
/// # Example
///
/// ```
/// use no_surprises::prelude::*;
/// use no_surprises::scales::major_scale;
///
/// let scale = major_scale(C4);
/// assert_eq!(scale.pitches(), &[C4, D4, E4, F4, G4, A4, B4, C5]);
/// ```
pub fn major_scale(root: Pitch) -> <MajorScaleQuality as ScaleQuality>::Pitches {
    MAJOR_SCALE_IN_INTERVALS.to_scale_in_pitches(root)
}

/// Creates a major scale using the steps-based implementation.
/// This is an alternative to the interval-based implementation.
///
/// # Arguments
///
/// * `root` - The root pitch to start the scale from
///
/// # Returns
///
/// A major scale represented as an array of pitches, including the octave
///
/// # Example
///
/// ```
/// use no_surprises::prelude::*;
/// use no_surprises::scales::major_scale_in_steps;
///
/// let scale = major_scale_in_steps(C4);
/// assert_eq!(scale.pitches(), &[C4, D4, E4, F4, G4, A4, B4, C5]);
/// ```
pub fn major_scale_in_steps(root: Pitch) -> <MajorScaleQuality as ScaleQuality>::Pitches {
    constants::MAJOR_SCALE_IN_STEPS.to_scale_in_pitches(root)
}

pub(crate) mod constants {
    use super::*;
    use crate::Step;
    use crate::prelude::*;

    /// The steps pattern for a major scale: whole, whole, half, whole, whole, whole, half
    pub const MAJOR_SCALE_STEPS: [Step; <MajorScaleQuality as ScaleQuality>::STEPS] =
        [WHOLE, WHOLE, HALF, WHOLE, WHOLE, WHOLE, HALF];

    /// The intervals pattern for a major scale from the root note
    pub const MAJOR_SCALE_INTERVALS: [Interval; <MajorScaleQuality as ScaleQuality>::INTERVALS] = [
        MAJOR_SECOND,
        MAJOR_THIRD,
        PERFECT_FOURTH,
        PERFECT_FIFTH,
        MAJOR_SIXTH,
        MAJOR_SEVENTH,
        PERFECT_OCTAVE,
    ];

    /// A major scale represented as intervals
    pub const MAJOR_SCALE_IN_INTERVALS: <MajorScaleQuality as ScaleQuality>::Intervals =
        <MajorScaleQuality as ScaleQuality>::Intervals::new(MAJOR_SCALE_INTERVALS);

    /// A major scale represented as steps
    #[allow(dead_code)]
    pub const MAJOR_SCALE_IN_STEPS: <MajorScaleQuality as ScaleQuality>::Steps =
        <MajorScaleQuality as ScaleQuality>::Steps::new(MAJOR_SCALE_STEPS);
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::prelude::*;

    /// Test that the major scale is created correctly from C4
    #[test]
    fn test_major_scale() {
        let scale = major_scale(C4);
        assert_eq!(scale.pitches(), &[C4, D4, E4, F4, G4, A4, B4, C5]);
    }

    /// Test that the steps-based implementation produces the same result
    #[test]
    fn test_major_scale_in_steps() {
        let scale = major_scale_in_steps(C4);
        assert_eq!(scale.pitches(), &[C4, D4, E4, F4, G4, A4, B4, C5]);
    }

    /// Test that both implementations produce the same result
    #[test]
    fn test_implementations_equivalent() {
        let interval_scale = major_scale(C4);
        let steps_scale = major_scale_in_steps(C4);
        assert_eq!(interval_scale.pitches(), steps_scale.pitches());
    }

    /// Test that the scale works with different root notes
    #[test]
    fn test_different_roots() {
        let g_scale = major_scale(G4);
        assert_eq!(g_scale.pitches(), &[G4, A4, B4, C5, D5, E5, FSHARP5, G5]);

        let f_scale = major_scale(F4);
        assert_eq!(f_scale.pitches(), &[F4, G4, A4, BFLAT4, C5, D5, E5, F5]);
    }

    /// Test that the scale maintains its pattern regardless of octave
    #[test]
    fn test_octave_independence() {
        let c4_scale = major_scale(C4);
        let c5_scale = major_scale(C5);

        // Compare the relative intervals between notes
        for i in 0..MajorScaleQuality::STEPS {
            let c4_interval = c4_scale.pitches()[i + 1] - c4_scale.pitches()[i];
            let c5_interval = c5_scale.pitches()[i + 1] - c5_scale.pitches()[i];
            assert_eq!(c4_interval, c5_interval);
        }
    }
}
