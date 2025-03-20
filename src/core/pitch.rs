use std::ops::{Add, Sub};

use super::{Interval, IntoIntervals, IntoSteps, Step};

#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct Pitch(u8);

impl Default for Pitch {
    #[inline]
    fn default() -> Self {
        Self(60)
    }
}

impl Pitch {
    #[inline]
    pub const fn new(semitones: u8) -> Self {
        Self(semitones)
    }

    #[inline]
    pub const fn semitones(&self) -> u8 {
        self.0
    }
}

impl Sub for Pitch {
    type Output = Step;

    #[inline]
    fn sub(self, other: Self) -> Self::Output {
        Step::new(self.0 - other.0)
    }
}

impl Add<Step> for Pitch {
    type Output = Self;

    #[inline]
    fn add(self, step: Step) -> Self::Output {
        Self(self.0 + step.semitones())
    }
}

impl<const N: usize> IntoIntervals for [Pitch; N] {
    fn into_intervals<const M: usize>(self) -> [Interval; M] {
        debug_assert!(M == N - 1);

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

impl<const N: usize> IntoSteps for [Pitch; N] {
    fn into_steps<const M: usize>(self) -> [Step; M] {
        debug_assert!(M == N - 1);

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

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_into_intervals() {
        let pitches = [Pitch::new(60), Pitch::new(62), Pitch::new(65), Pitch::new(69)];
        let intervals = pitches.into_intervals();
        assert_eq!(intervals, [Interval::new(2), Interval::new(5), Interval::new(9)]);
    }

    #[test]
    fn test_into_steps() {
        let pitches = [Pitch::new(60), Pitch::new(62), Pitch::new(65), Pitch::new(69)];
        let steps = pitches.into_steps();
        assert_eq!(steps, [Step::new(2), Step::new(3), Step::new(4)]);
    }

}