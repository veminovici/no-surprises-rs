//! Scale intervals module for handling scales represented as intervals
//!
//! This module provides functionality for working with scales represented as sequences
//! of intervals. It includes implementations for converting between different scale
//! representations (intervals, steps, and pitches).

use super::{
    IntoScaleInPitches, IntoScaleInSteps, Scale, ScaleInPitches, ScaleInSteps, ScaleQuality,
};
use crate::{Interval, IntoPitches, IntoSteps, Pitch};

/// A type alias for a scale represented as intervals
///
/// This type represents a scale where each element is an interval,
/// representing the distance between consecutive scale degrees.
///
/// # Type Parameters
///
/// * `Q` - The scale quality (e.g., major, minor)
/// * `N` - The number of intervals in the scale
pub type ScaleInIntervals<Q, const N: usize> = Scale<Q, Interval, N>;

impl<Q: ScaleQuality, const N: usize> ScaleInIntervals<Q, N> {
    /// Returns a reference to the intervals in the scale
    ///
    /// This function provides access to the underlying array of intervals that make up the scale.
    /// Each interval represents the distance between consecutive scale degrees.
    ///
    /// # Returns
    ///
    /// A reference to the array of intervals in the scale
    ///
    /// # Examples
    ///
    /// ```rust
    /// use no_surprises::prelude::*;
    /// use no_surprises::scales::{ScaleInIntervals, ScaleQuality};
    ///
    /// struct MajorScale;
    /// impl ScaleQuality for MajorScale {}
    ///
    /// let intervals = [MAJOR_SECOND, MAJOR_SECOND, MINOR_SECOND, MAJOR_SECOND, MAJOR_SECOND, MAJOR_SECOND, MINOR_SECOND];
    /// let scale = ScaleInIntervals::<MajorScale, 7>::new(intervals);
    /// assert_eq!(scale.intervals(), &intervals);
    /// ```
    #[inline]
    pub const fn intervals(&self) -> &[Interval; N] {
        &self.items
    }
}

/// Implementation of IntoScaleInSteps for scales represented as intervals
///
/// This implementation allows converting a scale from intervals to steps
/// by calculating the differences between consecutive intervals.
impl<Q: ScaleQuality, const N: usize> IntoScaleInSteps<Q> for ScaleInIntervals<Q, N> {
    /// Converts the scale from intervals to steps
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
    /// Panics if M != N (checked via debug_assert)
    #[inline]
    fn into_scale_in_steps<const M: usize>(self) -> ScaleInSteps<Q, M> {
        debug_assert!(
            M == N,
            "ScaleInIntervals and ScaleInSteps must have the same number of items. We got M={} and N={}",
            N,
            M
        );

        let steps = self.items.into_steps();
        Scale::new(steps)
    }
}

/// Implementation of IntoScaleInPitches for scales represented as intervals
///
/// This implementation allows converting a scale from intervals to pitches
/// by applying the intervals to a root pitch.
impl<Q: ScaleQuality, const N: usize> IntoScaleInPitches<Q> for ScaleInIntervals<Q, N> {
    /// Converts the scale from intervals to pitches
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
    ///
    /// # Panics
    ///
    /// Panics if M != N + 1 (checked via debug_assert)
    fn into_scale_in_pitches<const M: usize>(self, root: Pitch) -> ScaleInPitches<Q, M> {
        debug_assert!(
            M == N + 1,
            "ScaleInIntervals has one less item than ScaleInPitches. We got N={} and M={}",
            N,
            M
        );

        let pitches = self.items.into_pitches(root);
        Scale::new(pitches)
    }
}

#[cfg(test)]
mod tests {
    use crate::prelude::*;

    use super::*;

    /// A test scale quality for testing scale operations
    struct TestScaleQuality;
    impl ScaleQuality for TestScaleQuality {}

    #[test]
    fn test_intervals_accessor() {
        let intervals = [MAJOR_SECOND, MINOR_THIRD, MAJOR_THIRD];
        let scale = ScaleInIntervals::<TestScaleQuality, 3>::new(intervals);

        // Test that we can access the intervals
        assert_eq!(scale.intervals(), &intervals);

        // Test that the intervals are the same as the items
        assert_eq!(scale.intervals(), scale.items());
    }

    #[test]
    fn test_scale_in_intervals_into_scale_in_steps() {
        let intervals = [MAJOR_SECOND, MINOR_THIRD, MAJOR_THIRD];
        let scale_in_intervals = ScaleInIntervals::<TestScaleQuality, 3>::new(intervals);
        let scale_in_steps = scale_in_intervals.into_scale_in_steps();

        assert_eq!(scale_in_steps.items(), &[WHOLE, HALF, HALF]);
    }

    #[test]
    fn test_scale_in_intervals_into_scale_in_pitches() {
        let intervals = [MAJOR_SECOND, MINOR_THIRD, MAJOR_THIRD];
        let scale_in_intervals = ScaleInIntervals::<TestScaleQuality, 3>::new(intervals);
        let scale_in_pitches = scale_in_intervals.into_scale_in_pitches(C4);

        assert_eq!(scale_in_pitches.items(), &[C4, D4, DSHARP4, E4]);
    }
}
