//! Scale steps module for handling scales represented as steps
//!
//! This module provides functionality for working with scales represented as sequences
//! of steps. It includes implementations for converting between different scale
//! representations (steps, intervals, and pitches).

use super::{
    IntoScaleInIntervals, IntoScaleInPitches, Scale, ScaleInIntervals, ScaleInPitches, ScaleQuality,
};
use crate::{IntoIntervals, IntoPitches, Pitch, Step};

/// A type alias for a scale represented as steps
///
/// This type represents a scale where each element is a step,
/// representing the smallest unit of movement between consecutive scale degrees.
///
/// # Type Parameters
///
/// * `Q` - The scale quality (e.g., major, minor)
/// * `N` - The number of steps in the scale
pub type ScaleInSteps<Q, const N: usize> = Scale<Q, Step, N>;

impl<Q: ScaleQuality, const N: usize> ScaleInSteps<Q, N> {
    /// Returns a reference to the steps in the scale
    ///
    /// This function provides access to the underlying array of steps that make up the scale.
    /// Each step represents the smallest unit of movement between consecutive scale degrees.
    ///
    /// # Returns
    ///
    /// A reference to the array of steps in the scale
    ///
    /// # Examples
    ///
    /// ```rust
    /// use no_surprises::prelude::*;
    /// use no_surprises::scales::{ScaleInSteps, ScaleQuality};
    ///
    /// struct MajorScale;
    /// impl ScaleQuality for MajorScale {}
    ///
    /// let steps = [WHOLE, WHOLE, HALF, WHOLE, WHOLE, WHOLE, HALF];
    /// let scale = ScaleInSteps::<MajorScale, 7>::new(steps);
    /// assert_eq!(scale.steps(), &steps);
    /// ```
    #[inline]
    pub const fn steps(&self) -> &[Step; N] {
        &self.items
    }
}

/// Implementation of IntoScaleInIntervals for scales represented as steps
///
/// This implementation allows converting a scale from steps to intervals
/// by accumulating the steps to calculate the intervals.
impl<Q: ScaleQuality, const N: usize> IntoScaleInIntervals<Q> for ScaleInSteps<Q, N> {
    /// Converts the scale from steps to intervals
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
    /// Panics if M != N (checked via debug_assert)
    #[inline]
    fn into_scale_in_intervals<const M: usize>(&self) -> ScaleInIntervals<Q, M> {
        debug_assert!(
            M == N,
            "ScaleInSteps and ScaleInIntervals must have the same number of items. We got M={} and N={}",
            M,
            N
        );

        let intervals = self.items.into_intervals();
        Scale::new(intervals)
    }
}

/// Implementation of IntoScaleInPitches for scales represented as steps
///
/// This implementation allows converting a scale from steps to pitches
/// by applying the steps to a root pitch.
impl<Q: ScaleQuality, const N: usize> IntoScaleInPitches<Q> for ScaleInSteps<Q, N> {
    /// Converts the scale from steps to pitches
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
    fn into_scale_in_pitches<const M: usize>(&self, root: Pitch) -> ScaleInPitches<Q, M> {
        debug_assert!(
            M == N + 1,
            "ScaleInSteps has one less item than ScaleInPitches. We got N={} and M={}",
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
    fn test_steps_accessor() {
        let steps = [WHOLE, HALF, HALF];
        let scale = ScaleInSteps::<TestScaleQuality, 3>::new(steps);

        // Test that we can access the steps
        assert_eq!(scale.steps(), &steps);

        // Test that the steps are the same as the items
        assert_eq!(scale.steps(), scale.items());
    }

    #[test]
    fn test_scale_in_steps_into_scale_in_intervals() {
        let steps = [WHOLE, HALF, HALF];
        let scale_in_steps = ScaleInSteps::<TestScaleQuality, 3>::new(steps);
        let scale_in_intervals = scale_in_steps.into_scale_in_intervals();

        assert_eq!(
            scale_in_intervals.items(),
            &[MAJOR_SECOND, MINOR_THIRD, MAJOR_THIRD]
        );
    }

    #[test]
    fn test_scale_in_steps_into_scale_in_pitches() {
        let steps = [WHOLE, HALF, HALF];
        let scale_in_steps = ScaleInSteps::<TestScaleQuality, 3>::new(steps);
        let scale_in_pitches = scale_in_steps.into_scale_in_pitches(C4);

        assert_eq!(scale_in_pitches.items(), &[C4, D4, DSHARP4, E4]);
    }
}
