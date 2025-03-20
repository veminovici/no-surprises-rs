//! Pitch module for handling musical pitches
//!
//! A pitch represents a specific musical note in the MIDI system, where each pitch
//! is assigned a unique number (0-127). This module provides functionality for working
//! with pitches, including conversion to and from intervals and steps, and operations
//! on collections of pitches.

use std::ops::{Add, Sub};

use crate::C4;

use super::{Interval, IntoIntervals, IntoSteps, Step};

/// Represents a musical pitch in the MIDI system.
///
/// A pitch is a fundamental concept in music that represents a specific note.
/// In MIDI, pitches are represented as numbers from 0 to 127, where:
/// - 0-11: Octave -1 (C-1 to B-1)
/// - 12-23: Octave 0 (C0 to B0)
/// - 24-35: Octave 1 (C1 to B1)
/// - And so on...
///
/// The most common octave is octave 4, where middle C is 60.
#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct Pitch(u8);

impl Default for Pitch {
    /// Returns middle C (MIDI note 60) as the default value.
    #[inline]
    fn default() -> Self {
        C4
    }
}

impl Pitch {
    /// Creates a new pitch with the given MIDI note number.
    ///
    /// # Arguments
    ///
    /// * `semitones` - The MIDI note number (0-127)
    ///
    /// # Returns
    ///
    /// A new Pitch with the specified MIDI note number
    #[inline]
    pub const fn new(semitones: u8) -> Self {
        Self(semitones)
    }

    /// Returns the MIDI note number of this pitch.
    ///
    /// # Returns
    ///
    /// The MIDI note number as a u8 (0-127)
    #[inline]
    pub const fn semitones(&self) -> u8 {
        self.0
    }
}

impl Add<Step> for Pitch {
    type Output = Self;

    /// Adds a step to a pitch.
    ///
    /// # Arguments
    ///
    /// * `self` - The pitch to add to
    /// * `step` - The step to add
    ///
    /// # Returns
    ///
    /// A new pitch with the sum of the MIDI note numbers
    #[inline]
    fn add(self, step: Step) -> Self::Output {
        Self(self.0 + step.semitones())
    }
}

impl Sub<Step> for Pitch {
    type Output = Self;

    /// Subtracts a step from a pitch.
    ///
    /// # Arguments
    ///
    /// * `self` - The pitch to subtract from
    /// * `step` - The step to subtract
    ///
    /// # Returns
    ///
    /// A new pitch with the difference of the MIDI note numbers
    #[inline]
    fn sub(self, step: Step) -> Self::Output {
        Self(self.0 - step.semitones())
    }
}

impl Sub for Pitch {
    type Output = Step;

    /// Subtracts two pitches to get the step between them.
    ///
    /// # Arguments
    ///
    /// * `self` - The first pitch
    /// * `other` - The second pitch to subtract
    ///
    /// # Returns
    ///
    /// A step representing the difference between the pitches
    #[inline]
    fn sub(self, other: Self) -> Self::Output {
        Step::new(self.0 - other.0)
    }
}

/// Implementation of IntoIntervals for arrays of pitches.
///
/// This allows converting a sequence of pitches into a sequence of intervals
/// by calculating the distance from the root pitch.
impl<const N: usize> IntoIntervals for [Pitch; N] {
    /// Converts an array of pitches into an array of intervals.
    ///
    /// # Arguments
    ///
    /// * `self` - The array of pitches to convert
    ///
    /// # Returns
    ///
    /// An array of intervals where each interval is the distance from the root pitch
    ///
    /// # Panics
    ///
    /// Panics if M != N - 1 (checked via debug_assert)
    fn into_intervals<const M: usize>(self) -> [Interval; M] {
        if N == 0 {
            return [Interval::default(); M];
        }

        debug_assert!(
            M == N - 1,
            "For pitches into intervals, M must be equal to N - 1, got M={} and N={}",
            M,
            N
        );

        let mut intervals = [Interval::default(); M];

        let root = self[0];

        self.iter()
            .skip(1)
            .map(|pitch| Interval::from(*pitch - root))
            .enumerate()
            .for_each(|(i, interval)| {
                intervals[i] = interval;
            });

        intervals
    }
}

/// Implementation of IntoSteps for arrays of pitches.
///
/// This allows converting a sequence of pitches into a sequence of steps
/// by calculating the differences between consecutive pitches.
impl<const N: usize> IntoSteps for [Pitch; N] {
    /// Converts an array of pitches into an array of steps.
    ///
    /// # Arguments
    ///
    /// * `self` - The array of pitches to convert
    ///
    /// # Returns
    ///
    /// An array of steps where each step is the difference between consecutive pitches
    ///
    /// # Panics
    ///
    /// Panics if M != N - 1 (checked via debug_assert)
    fn into_steps<const M: usize>(self) -> [Step; M] {
        if N == 0 {
            return [Step::default(); M];
        }

        debug_assert!(
            M == N - 1,
            "For pitches into steps, M must be equal to N - 1, got M={} and N={}",
            M,
            N
        );

        let mut steps = [Step::default(); M];

        self.windows(2)
            .map(|pitches| pitches[1] - pitches[0])
            .enumerate()
            .for_each(|(i, step)| {
                steps[i] = step;
            });

        steps
    }
}

/// Constants for common musical pitches in octave 4.
pub(crate) mod constants {
    use super::Pitch;
    use paste::paste;

    macro_rules! define_octave {
        ($octave:expr) => {
            paste! {
                // Octave $octave pitches.

                pub const [<C $octave>]: Pitch = Pitch::new(($octave + 1) * crate::core::SEMITONES_IN_OCTAVE);
                pub const [<CSHARP $octave>]: Pitch = Pitch::new(($octave + 1) * crate::core::SEMITONES_IN_OCTAVE + 1);
                pub const [<DFLAT $octave>]: Pitch = [<CSHARP $octave>];
                pub const [<D $octave>]: Pitch = Pitch::new(($octave + 1) * crate::core::SEMITONES_IN_OCTAVE + 2);
                pub const [<DSHARP $octave>]: Pitch = Pitch::new(($octave + 1) * crate::core::SEMITONES_IN_OCTAVE + 3);
                pub const [<EFLAT $octave>]: Pitch = [<DSHARP $octave>];
                pub const [<E $octave>]: Pitch = Pitch::new(($octave + 1) * crate::core::SEMITONES_IN_OCTAVE + 4);
                pub const [<F $octave>]: Pitch = Pitch::new(($octave + 1) * crate::core::SEMITONES_IN_OCTAVE + 5);
                pub const [<FSHARP $octave>]: Pitch = Pitch::new(($octave + 1) * crate::core::SEMITONES_IN_OCTAVE + 6);
                pub const [<GFLAT $octave>]: Pitch = [<FSHARP $octave>];
                pub const [<G $octave>]: Pitch = Pitch::new(($octave + 1) * crate::core::SEMITONES_IN_OCTAVE + 7);
                pub const [<GSHARP $octave>]: Pitch = Pitch::new(($octave + 1) * crate::core::SEMITONES_IN_OCTAVE + 8);
                pub const [<AFLAT $octave>]: Pitch = [<GSHARP $octave>];
                pub const [<A $octave>]: Pitch = Pitch::new(($octave + 1) * crate::core::SEMITONES_IN_OCTAVE + 9);
                pub const [<ASHARP $octave>]: Pitch = Pitch::new(($octave + 1) * crate::core::SEMITONES_IN_OCTAVE + 10);
                pub const [<BFLAT $octave>]: Pitch = [<ASHARP $octave>];
                pub const [<B $octave>]: Pitch = Pitch::new(($octave + 1) * crate::core::SEMITONES_IN_OCTAVE + 11);
            }
        };
    }

    // Base octave pitches (C through B)
    pub const C: Pitch = Pitch::new(0);
    pub const CSHARP: Pitch = Pitch::new(1);
    pub const DFLAT: Pitch = CSHARP;
    pub const D: Pitch = Pitch::new(2);
    pub const DSHARP: Pitch = Pitch::new(3);
    pub const EFLAT: Pitch = DSHARP;
    pub const E: Pitch = Pitch::new(4);
    pub const F: Pitch = Pitch::new(5);
    pub const FSHARP: Pitch = Pitch::new(6);
    pub const GFLAT: Pitch = FSHARP;
    pub const G: Pitch = Pitch::new(7);
    pub const GSHARP: Pitch = Pitch::new(8);
    pub const AFLAT: Pitch = GSHARP;
    pub const A: Pitch = Pitch::new(9);
    pub const ASHARP: Pitch = Pitch::new(10);
    pub const BFLAT: Pitch = ASHARP;
    pub const B: Pitch = Pitch::new(11);

    define_octave!(0);
    define_octave!(1);
    define_octave!(2);
    define_octave!(3);
    define_octave!(4);
    define_octave!(5);
    define_octave!(6);
    define_octave!(7);
    define_octave!(8);
    define_octave!(9);
}

#[cfg(test)]
mod tests {
    use super::constants::*;
    use super::*;

    #[test]
    fn test_pitch_creation() {
        let pitch = Pitch::new(60);
        assert_eq!(pitch.semitones(), 60);
    }

    #[test]
    fn test_pitch_default() {
        let pitch = Pitch::default();
        assert_eq!(pitch.semitones(), 60); // C4
    }

    #[test]
    fn test_pitch_addition() {
        let pitch = Pitch::new(60);
        let step = Step::new(2);
        let result = pitch + step;
        assert_eq!(result.semitones(), 62);
    }

    #[test]
    fn test_pitch_subtraction() {
        let pitch = Pitch::new(64);
        let step = Step::new(3);
        let result = pitch - step;
        assert_eq!(result.semitones(), 61);
    }

    #[test]
    fn test_pitch_difference() {
        let pitch1 = Pitch::new(64);
        let pitch2 = Pitch::new(60);
        let step = pitch1 - pitch2;
        assert_eq!(step.semitones(), 4);
    }

    #[test]
    fn test_pitch_comparison() {
        let pitch1 = Pitch::new(60);
        let pitch2 = Pitch::new(64);
        assert!(pitch1 < pitch2);
        assert!(pitch2 > pitch1);
        assert_eq!(pitch1, Pitch::new(60));
    }

    #[test]
    fn test_into_intervals() {
        let pitches = [
            Pitch::new(60),
            Pitch::new(62),
            Pitch::new(65),
            Pitch::new(69),
        ];
        let intervals = pitches.into_intervals();
        assert_eq!(
            intervals,
            [Interval::new(2), Interval::new(5), Interval::new(9)]
        );
    }

    #[test]
    fn test_into_steps() {
        let pitches = [
            Pitch::new(60),
            Pitch::new(62),
            Pitch::new(65),
            Pitch::new(69),
        ];
        let steps = pitches.into_steps();
        assert_eq!(steps, [Step::new(2), Step::new(3), Step::new(4)]);
    }

    #[test]
    fn test_constants() {
        assert_eq!(C4.semitones(), 60);
        assert_eq!(CSHARP4.semitones(), 61);
        assert_eq!(DFLAT4.semitones(), 61);
        assert_eq!(D4.semitones(), 62);
        assert_eq!(DSHARP4.semitones(), 63);
        assert_eq!(EFLAT4.semitones(), 63);
        assert_eq!(E4.semitones(), 64);
        assert_eq!(F4.semitones(), 65);
        assert_eq!(FSHARP4.semitones(), 66);
        assert_eq!(GFLAT4.semitones(), 66);
        assert_eq!(G4.semitones(), 67);
        assert_eq!(GSHARP4.semitones(), 68);
        assert_eq!(AFLAT4.semitones(), 68);
        assert_eq!(A4.semitones(), 69);
        assert_eq!(ASHARP4.semitones(), 70);
        assert_eq!(BFLAT4.semitones(), 70);
        assert_eq!(B4.semitones(), 71);
        assert_eq!(C5.semitones(), 72);
    }

    #[test]
    fn test_empty_pitches() {
        let pitches: [Pitch; 0] = [];
        let intervals: [Interval; 0] = pitches.into_intervals();
        assert_eq!(intervals.len(), 0);

        let steps: [Step; 0] = pitches.into_steps();
        assert_eq!(steps.len(), 0);
    }

    #[test]
    fn test_single_pitch() {
        let pitches = [Pitch::new(60)];
        let intervals: [Interval; 0] = pitches.into_intervals();
        assert_eq!(intervals.len(), 0);

        let steps: [Step; 0] = pitches.into_steps();
        assert_eq!(steps.len(), 0);
    }
}
