//! Scale pitches module for handling scales represented as pitches
//!
//! This module provides functionality for working with scales represented as sequences
//! of pitches. It includes implementations for converting between different scale
//! representations (pitches, intervals, and steps).

use super::{
    Scale, ScaleInIntervals, ScaleInSteps, ScaleQuality, ToScaleInIntervals, ToScaleInSteps,
};
use crate::{IntoIntervals, IntoSteps, Pitch};

/// A type alias for a scale represented as pitches
///
/// This type represents a scale where each element is a pitch,
/// representing the actual notes in the scale.
///
/// # Type Parameters
///
/// * `Q` - The scale quality (e.g., major, minor)
/// * `N` - The number of pitches in the scale
pub type ScaleInPitches<Q, const N: usize> = Scale<Q, Pitch, N>;

impl<Q: ScaleQuality, const N: usize> ScaleInPitches<Q, N> {
    /// Returns a reference to the pitches in the scale
    ///
    /// This function provides access to the underlying array of pitches that make up the scale.
    /// Each pitch represents an actual note in the scale, with its specific frequency and position
    /// in the musical scale.
    ///
    /// # Returns
    ///
    /// A reference to the array of pitches in the scale
    ///
    /// # Examples
    ///
    /// ```rust
    /// use no_surprises::prelude::*;
    /// use no_surprises::scales::{ScaleInPitches, ScaleInSteps, ScaleInIntervals, ScaleQuality};
    ///
    /// struct MajorScale;
    /// impl ScaleQuality for MajorScale {
    ///    type Steps = ScaleInSteps<Self, { Self::STEPS }>;    
    ///    type Intervals = ScaleInIntervals<Self, { Self::INTERVALS }>;    
    ///    type Pitches = ScaleInPitches<Self, { Self::PITCHES }>;
    ///    const STEPS: usize = 7;
    ///    const INTERVALS: usize = Self::STEPS;
    ///    const PITCHES: usize = Self::STEPS + 1;
    /// }
    ///
    /// let pitches = [C4, D4, E4, F4, G4, A4, B4, C5];
    /// let scale = ScaleInPitches::<MajorScale, 8>::new(pitches);
    /// assert_eq!(scale.pitches(), &pitches);
    /// ```
    #[inline]
    pub const fn pitches(&self) -> &[Pitch; N] {
        &self.items
    }
}

/// Implementation of ToScaleInIntervals for scales represented as pitches
///
/// This implementation allows converting a scale from pitches to intervals
/// by calculating the distances between consecutive pitches.
impl<Q: ScaleQuality, const N: usize> ToScaleInIntervals<Q> for ScaleInPitches<Q, N> {
    /// Converts the scale from pitches to intervals
    ///
    /// # Type Parameters
    ///
    /// * `M` - The number of intervals in the resulting scale
    ///
    /// # Returns
    ///
    /// A scale containing the intervals between scale degrees
    ///
    /// # Panics
    ///
    /// Panics if M != N - 1 (checked via debug_assert)
    #[inline]
    fn to_scale_in_intervals<const M: usize>(&self) -> ScaleInIntervals<Q, M> {
        debug_assert!(
            M == N - 1,
            "ScaleInPitches has one more item than ScaleInIntervals. We got N={} and M={}",
            N,
            M
        );

        let intervals = self.items.into_intervals();
        Scale::new(intervals)
    }
}

/// Implementation of ToScaleInSteps for scales represented as pitches
///
/// This implementation allows converting a scale from pitches to steps
/// by calculating the differences between consecutive pitches.
impl<Q: ScaleQuality, const N: usize> ToScaleInSteps<Q> for ScaleInPitches<Q, N> {
    /// Converts the scale from pitches to steps
    ///
    /// # Type Parameters
    ///
    /// * `M` - The number of steps in the resulting scale
    ///
    /// # Returns
    ///
    /// A scale containing the steps between scale degrees
    ///
    /// # Panics
    ///
    /// Panics if M != N - 1 (checked via debug_assert)
    #[inline]
    fn to_scale_in_steps<const M: usize>(&self) -> ScaleInSteps<Q, M> {
        debug_assert!(
            M == N - 1,
            "ScaleInPitches has one more item than ScaleInSteps. We got N={} and M={}",
            N,
            M
        );

        let steps = self.items.into_steps();
        Scale::new(steps)
    }
}

#[cfg(test)]
mod tests {
    use crate::prelude::*;

    use super::*;

    /// A test scale quality for testing scale operations
    struct TestScaleQuality;
    impl ScaleQuality for TestScaleQuality {
        /// The type alias for a major scale represented as steps
        type Steps = ScaleInSteps<Self, { Self::STEPS }>;

        /// The type alias for a major scale represented as intervals
        type Intervals = ScaleInIntervals<Self, { Self::INTERVALS }>;

        /// The type alias for a major scale represented as pitches
        type Pitches = ScaleInPitches<Self, { Self::PITCHES }>;

        /// The number of steps in a major scale (excluding the octave)
        const STEPS: usize = 3;

        /// The number of intervals in a major scale (excluding the octave)
        const INTERVALS: usize = Self::STEPS;

        /// The number of pitches in a major scale (including the octave)
        const PITCHES: usize = Self::STEPS + 1;
    }

    #[test]
    fn test_pitches_accessor() {
        let pitches = [C4, D4, DSHARP4, E4];
        let scale = ScaleInPitches::<TestScaleQuality, 4>::new(pitches);

        // Test that we can access the pitches
        assert_eq!(scale.pitches(), &pitches);

        // Test that the pitches are the same as the items
        assert_eq!(scale.pitches(), scale.items());
    }

    #[test]
    fn test_scale_in_pitches_to_scale_in_intervals() {
        let pitches = [C4, D4, DSHARP4, E4];
        let scale_in_pitches = ScaleInPitches::<TestScaleQuality, 4>::new(pitches);
        let scale_in_intervals = scale_in_pitches.to_scale_in_intervals();

        assert_eq!(
            scale_in_intervals.items(),
            &[MAJOR_SECOND, MINOR_THIRD, MAJOR_THIRD]
        );
    }

    #[test]
    fn test_scale_in_pitches_to_scale_in_steps() {
        let pitches = [C4, D4, DSHARP4, E4];
        let scale_in_pitches = ScaleInPitches::<TestScaleQuality, 4>::new(pitches);
        let scale_in_steps = scale_in_pitches.to_scale_in_steps();

        assert_eq!(scale_in_steps.items(), &[WHOLE, HALF, HALF]);
    }
}
