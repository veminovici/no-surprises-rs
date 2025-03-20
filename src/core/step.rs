//! Step module for handling musical steps (the distance between two adjacent pitches)
//!
//! A step represents the distance between two adjacent pitches in semitones.
//! This module provides functionality for working with steps, including conversion
//! to and from intervals, and operations on collections of steps.

use crate::UNISON;

use super::{Interval, IntoIntervals, IntoPitches, Pitch};

/// Represents a musical step, which is the distance between two adjacent pitches in semitones.
///
/// A step is the smallest unit of pitch movement in music, where one semitone
/// represents the smallest possible step in standard Western music.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub struct Step(u8);

impl Step {
    /// Creates a new step with the given number of semitones.
    ///
    /// # Arguments
    ///
    /// * `semitones` - The number of semitones in the step
    ///
    /// # Returns
    ///
    /// A new Step with the specified number of semitones
    #[inline]
    pub const fn new(semitones: u8) -> Self {
        Self(semitones)
    }

    /// Returns the number of semitones in this step.
    ///
    /// # Returns
    ///
    /// The number of semitones as a u8
    #[inline]
    pub const fn semitones(&self) -> u8 {
        self.0
    }
}

impl Default for Step {
    /// Returns a unison step (0 semitones) as the default value.
    #[inline]
    fn default() -> Self {
        UNISON
    }
}

impl From<Interval> for Step {
    /// Converts an interval into a step.
    ///
    /// # Arguments
    ///
    /// * `interval` - The interval to convert
    ///
    /// # Returns
    ///
    /// A step with the same number of semitones as the interval
    #[inline]
    fn from(interval: Interval) -> Self {
        Step(interval.semitones())
    }
}

/// Implementation of IntoIntervals for arrays of steps.
///
/// This allows converting a sequence of steps into a sequence of intervals
/// by accumulating the steps.
impl<const N: usize> IntoIntervals for [Step; N] {
    /// Converts an array of steps into an array of intervals.
    ///
    /// # Arguments
    ///
    /// * `self` - The array of steps to convert
    ///
    /// # Returns
    ///
    /// An array of intervals where each interval is the sum of all previous steps
    ///
    /// # Panics
    ///
    /// Panics if M != N (checked via debug_assert)
    fn into_intervals<const M: usize>(self) -> [Interval; M] {
        debug_assert!(M == N);

        let scan_callback = |state: &mut Option<Interval>, step: &Step| -> Option<Interval> {
            let interval = state.map_or(Interval::from(*step), |prev| prev + *step);
            *state = Some(interval);
            Some(interval)
        };

        let mut intervals = [Interval::default(); M];

        self.iter()
            .scan(None::<Interval>, scan_callback)
            .enumerate()
            .for_each(|(i, item)| {
                intervals[i] = item;
            });

        intervals
    }
}

/// Implementation of IntoPitches for arrays of steps.
///
/// This allows converting a sequence of steps into a sequence of pitches
/// by applying the steps to a root pitch.
impl<const N: usize> IntoPitches for [Step; N] {
    /// Converts an array of steps into an array of pitches.
    ///
    /// # Arguments
    ///
    /// * `self` - The array of steps to convert
    /// * `root` - The root pitch to start from
    ///
    /// # Returns
    ///
    /// An array of pitches where each pitch is the root pitch plus the sum of all previous steps
    ///
    /// # Panics
    ///
    /// Panics if M != N + 1 (checked via debug_assert)
    fn into_pitches<const M: usize>(self, root: Pitch) -> [Pitch; M] {
        debug_assert!(M == N + 1);

        let scan_state = |last: &mut Pitch, step: &Step| -> Option<Pitch> {
            let pitch = *last + *step;
            *last = pitch;
            Some(pitch)
        };

        let rest = self.iter().scan(root, scan_state);

        let mut pitches = [Pitch::default(); M];

        ::std::iter::once(root)
            .chain(rest)
            .enumerate()
            .for_each(|(i, pitch)| {
                pitches[i] = pitch;
            });

        pitches
    }
}

/// Constants for common musical steps.
pub(crate) mod constants {
    use super::Step;
    /// A unison step (0 semitones)
    pub const UNISON: Step = Step::new(0);
    /// A half step (1 semitone)
    pub const HALF: Step = Step::new(1);
    /// A whole step (2 semitones)
    pub const WHOLE: Step = Step::new(2);
    /// A whole and half step (3 semitones)
    pub const WHOLE_AND_HALF: Step = Step::new(3);

    /// Alternative name for a half step (1 semitone)
    pub const SEMITONE: Step = Step::new(1);
    /// Alternative name for a whole step (2 semitones)
    pub const TONE: Step = Step::new(2);
}

#[cfg(test)]
mod tests {
    use super::constants::*;
    use super::*;

    #[test]
    fn test_step_creation() {
        let step = Step::new(2);
        assert_eq!(step.semitones(), 2);
    }

    #[test]
    fn test_step_default() {
        let step = Step::default();
        assert_eq!(step.semitones(), 0);
    }

    #[test]
    fn test_step_from_interval() {
        let interval = Interval::new(3);
        let step = Step::from(interval);
        assert_eq!(step.semitones(), 3);
    }

    #[test]
    fn test_step_comparison() {
        let step1 = Step::new(2);
        let step2 = Step::new(3);
        assert!(step1 < step2);
        assert!(step2 > step1);
        assert_eq!(step1, Step::new(2));
    }

    #[test]
    fn test_into_intervals() {
        let steps = [Step::new(2), Step::new(3), Step::new(4)];
        let intervals = steps.into_intervals();
        assert_eq!(
            intervals,
            [Interval::new(2), Interval::new(5), Interval::new(9)]
        );
    }

    #[test]
    fn test_into_pitches() {
        let steps = [Step::new(2), Step::new(3), Step::new(4)];
        let pitches = steps.into_pitches(Pitch::new(60));
        assert_eq!(
            pitches,
            [
                Pitch::new(60),
                Pitch::new(62),
                Pitch::new(65),
                Pitch::new(69)
            ]
        );
    }

    #[test]
    fn test_constants() {
        assert_eq!(UNISON.semitones(), 0);
        assert_eq!(HALF.semitones(), 1);
        assert_eq!(WHOLE.semitones(), 2);
        assert_eq!(WHOLE_AND_HALF.semitones(), 3);
        assert_eq!(SEMITONE.semitones(), 1);
        assert_eq!(TONE.semitones(), 2);
    }

    #[test]
    fn test_empty_steps() {
        let steps: [Step; 0] = [];
        let intervals: [Interval; 0] = steps.into_intervals();
        assert_eq!(intervals.len(), 0);
    }

    #[test]
    fn test_single_step() {
        let steps = [Step::new(2)];
        let intervals = steps.into_intervals();
        assert_eq!(intervals, [Interval::new(2)]);

        let pitches = steps.into_pitches(Pitch::new(60));
        assert_eq!(pitches, [Pitch::new(60), Pitch::new(62)]);
    }
}
