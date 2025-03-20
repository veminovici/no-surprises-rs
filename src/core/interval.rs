//! Interval module for handling musical intervals (the distance between two pitches)
//!
//! An interval represents the distance between two pitches in semitones.
//! This module provides functionality for working with intervals, including conversion
//! to and from steps, and operations on collections of intervals.

use std::ops::{Add, Sub};

use crate::PERFECT_UNISON;

use super::{IntoPitches, IntoSteps, Pitch, Step};

/// Represents a musical interval, which is the distance between two pitches in semitones.
///
/// An interval is a fundamental concept in music theory that describes the distance
/// between two pitches. The smallest interval is a semitone (1), and larger intervals
/// are built by combining semitones.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub struct Interval(u8);

impl Interval {
    /// Creates a new interval with the given number of semitones.
    ///
    /// # Arguments
    ///
    /// * `semitones` - The number of semitones in the interval
    ///
    /// # Returns
    ///
    /// A new Interval with the specified number of semitones
    #[inline]
    pub const fn new(semitones: u8) -> Self {
        Self(semitones)
    }

    /// Returns the number of semitones in this interval.
    ///
    /// # Returns
    ///
    /// The number of semitones as a u8
    #[inline]
    pub const fn semitones(&self) -> u8 {
        self.0
    }
}

impl Default for Interval {
    /// Returns a perfect unison (0 semitones) as the default value.
    #[inline]
    fn default() -> Self {
        PERFECT_UNISON
    }
}

impl From<Step> for Interval {
    /// Converts a step into an interval.
    ///
    /// # Arguments
    ///
    /// * `step` - The step to convert
    ///
    /// # Returns
    ///
    /// An interval with the same number of semitones as the step
    #[inline]
    fn from(step: Step) -> Self {
        Self(step.semitones())
    }
}

impl Add<Step> for Interval {
    type Output = Self;

    /// Adds a step to an interval.
    ///
    /// # Arguments
    ///
    /// * `self` - The interval to add to
    /// * `step` - The step to add
    ///
    /// # Returns
    ///
    /// A new interval with the sum of the semitones
    #[inline]
    fn add(self, step: Step) -> Self::Output {
        Self(self.0 + step.semitones())
    }
}

impl Sub<Step> for Interval {
    type Output = Self;

    /// Subtracts a step from an interval.
    ///
    /// # Arguments
    ///
    /// * `self` - The interval to subtract from
    /// * `step` - The step to subtract
    ///
    /// # Returns
    ///
    /// A new interval with the difference of the semitones
    #[inline]
    fn sub(self, step: Step) -> Self::Output {
        Self(self.0 - step.semitones())
    }
}

impl Sub for Interval {
    type Output = Step;

    /// Subtracts two intervals to get the step between them.
    ///
    /// # Arguments
    ///
    /// * `self` - The first interval
    /// * `other` - The second interval to subtract
    ///
    /// # Returns
    ///
    /// A step representing the difference between the intervals
    #[inline]
    fn sub(self, other: Self) -> Self::Output {
        Step::new(self.0 - other.0)
    }
}

/// Implementation of IntoSteps for arrays of intervals.
///
/// This allows converting a sequence of intervals into a sequence of steps
/// by calculating the differences between consecutive intervals.
impl<const N: usize> IntoSteps for [Interval; N] {
    /// Converts an array of intervals into an array of steps.
    ///
    /// # Arguments
    ///
    /// * `self` - The array of intervals to convert
    ///
    /// # Returns
    ///
    /// An array of steps where each step is the difference between consecutive intervals
    ///
    /// # Panics
    ///
    /// Panics if M != N (checked via debug_assert)
    #[inline]
    fn into_steps<const M: usize>(self) -> [Step; M] {
        debug_assert!(
            M == N,
            "For intervals into steps, M must be equal to N, got M={} and N={}",
            M,
            N
        );

        let scan_state = |last: &mut Option<Interval>, item: Interval| -> Option<Step> {
            let step = last.map_or(Step::from(item), |prev| item - prev);
            *last = Some(item);
            Some(step)
        };

        let mut steps = [Step::default(); M];

        self.into_iter()
            .scan(None::<Interval>, scan_state)
            .enumerate()
            .for_each(|(i, item)| {
                steps[i] = item;
            });

        steps
    }
}

/// Implementation of IntoPitches for arrays of intervals.
///
/// This allows converting a sequence of intervals into a sequence of pitches
/// by applying the intervals to a root pitch.
impl<const N: usize> IntoPitches for [Interval; N] {
    /// Converts an array of intervals into an array of pitches.
    ///
    /// # Arguments
    ///
    /// * `self` - The array of intervals to convert
    /// * `root` - The root pitch to start from
    ///
    /// # Returns
    ///
    /// An array of pitches where each pitch is the root pitch plus the corresponding interval
    ///
    /// # Panics
    ///
    /// Panics if M != N + 1 (checked via debug_assert)
    #[inline]
    fn into_pitches<const M: usize>(self, root: Pitch) -> [Pitch; M] {
        debug_assert!(
            M == N + 1,
            "For intervals into pitches, M must be equal to N + 1, got M={} and N={}",
            M,
            N
        );

        let mut pitches = [root; M];

        self.into_iter()
            .map(|item| root + Step::from(item))
            .enumerate()
            .for_each(|(i, pitch)| {
                pitches[i + 1] = pitch;
            });

        pitches
    }
}

/// Constants for common musical intervals.
pub(crate) mod constants {
    use super::Interval;

    /// Perfect unison (0 semitones)
    pub const PERFECT_UNISON: Interval = Interval::new(0);
    /// Minor second (1 semitone)
    pub const MINOR_SECOND: Interval = Interval::new(1);
    /// Major second (2 semitones)
    pub const MAJOR_SECOND: Interval = Interval::new(2);
    /// Minor third (3 semitones)
    pub const MINOR_THIRD: Interval = Interval::new(3);
    /// Major third (4 semitones)
    pub const MAJOR_THIRD: Interval = Interval::new(4);
    /// Perfect fourth (5 semitones)
    pub const PERFECT_FOURTH: Interval = Interval::new(5);
    /// Augmented fourth (6 semitones)
    pub const AUGMENTED_FOURTH: Interval = Interval::new(6);
    /// Diminished fifth (6 semitones)
    pub const DIMINISHED_FIFTH: Interval = Interval::new(6);
    /// Perfect fifth (7 semitones)
    pub const PERFECT_FIFTH: Interval = Interval::new(7);
    /// Minor sixth (8 semitones)
    pub const MINOR_SIXTH: Interval = Interval::new(8);
    /// Major sixth (9 semitones)
    pub const MAJOR_SIXTH: Interval = Interval::new(9);
    /// Minor seventh (10 semitones)
    pub const MINOR_SEVENTH: Interval = Interval::new(10);
    /// Major seventh (11 semitones)
    pub const MAJOR_SEVENTH: Interval = Interval::new(11);
    /// Perfect octave (12 semitones)
    pub const PERFECT_OCTAVE: Interval = Interval::new(12);
}

#[cfg(test)]
mod tests {
    use super::constants::*;
    use super::*;

    #[test]
    fn test_interval_creation() {
        let interval = Interval::new(4);
        assert_eq!(interval.semitones(), 4);
    }

    #[test]
    fn test_interval_default() {
        let interval = Interval::default();
        assert_eq!(interval.semitones(), 0);
    }

    #[test]
    fn test_interval_from_step() {
        let step = Step::new(3);
        let interval = Interval::from(step);
        assert_eq!(interval.semitones(), 3);
    }

    #[test]
    fn test_interval_addition() {
        let interval = Interval::new(4);
        let step = Step::new(2);
        let result = interval + step;
        assert_eq!(result.semitones(), 6);
    }

    #[test]
    fn test_interval_subtraction() {
        let interval = Interval::new(7);
        let step = Step::new(3);
        let result = interval - step;
        assert_eq!(result.semitones(), 4);
    }

    #[test]
    fn test_interval_difference() {
        let interval1 = Interval::new(7);
        let interval2 = Interval::new(4);
        let step = interval1 - interval2;
        assert_eq!(step.semitones(), 3);
    }

    #[test]
    fn test_interval_comparison() {
        let interval1 = Interval::new(4);
        let interval2 = Interval::new(7);
        assert!(interval1 < interval2);
        assert!(interval2 > interval1);
        assert_eq!(interval1, Interval::new(4));
    }

    #[test]
    fn test_into_steps() {
        let intervals = [Interval::new(2), Interval::new(5), Interval::new(9)];
        let steps = intervals.into_steps();
        assert_eq!(steps, [Step::new(2), Step::new(3), Step::new(4)]);
    }

    #[test]
    fn test_into_pitches() {
        let intervals = [Interval::new(2), Interval::new(5), Interval::new(9)];
        let pitches = intervals.into_pitches(Pitch::new(60));
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
        assert_eq!(PERFECT_UNISON.semitones(), 0);
        assert_eq!(MINOR_SECOND.semitones(), 1);
        assert_eq!(MAJOR_SECOND.semitones(), 2);
        assert_eq!(MINOR_THIRD.semitones(), 3);
        assert_eq!(MAJOR_THIRD.semitones(), 4);
        assert_eq!(PERFECT_FOURTH.semitones(), 5);
        assert_eq!(AUGMENTED_FOURTH.semitones(), 6);
        assert_eq!(DIMINISHED_FIFTH.semitones(), 6);
        assert_eq!(PERFECT_FIFTH.semitones(), 7);
        assert_eq!(MINOR_SIXTH.semitones(), 8);
        assert_eq!(MAJOR_SIXTH.semitones(), 9);
        assert_eq!(MINOR_SEVENTH.semitones(), 10);
        assert_eq!(MAJOR_SEVENTH.semitones(), 11);
        assert_eq!(PERFECT_OCTAVE.semitones(), 12);
    }

    #[test]
    fn test_empty_intervals() {
        let intervals: [Interval; 0] = [];
        let steps: [Step; 0] = intervals.into_steps();
        assert_eq!(steps.len(), 0);
    }

    #[test]
    fn test_single_interval() {
        let intervals = [Interval::new(4)];
        let steps = intervals.into_steps();
        assert_eq!(steps, [Step::new(4)]);

        let pitches = intervals.into_pitches(Pitch::new(60));
        assert_eq!(pitches, [Pitch::new(60), Pitch::new(64)]);
    }
}
