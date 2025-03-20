use std::ops::{Add, Sub};

use super::{IntoPitches, IntoSteps, Pitch, Step};

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub struct Interval(u8);

impl Interval {
    #[inline]
    pub const fn new(semitones: u8) -> Self {
        Self(semitones)
    }

    #[inline]
    pub const fn semitones(&self) -> u8 {
        self.0
    }
}

impl Default for Interval {
    #[inline]
    fn default() -> Self {
        Self(0)
    }
}

impl From<Step> for Interval {
    #[inline]
    fn from(step: Step) -> Self {
        Self(step.semitones())
    }
}

impl Add<Step> for Interval {
    type Output = Self;

    #[inline]
    fn add(self, step: Step) -> Self::Output {
        Self(self.0 + step.semitones())
    }
}

impl Sub<Interval> for Interval {
    type Output = Self;

    #[inline]
    fn sub(self, other: Self) -> Self::Output {
        Self(self.0 - other.0)
    }
}

impl<const N: usize> IntoSteps for [Interval; N] {
    fn into_steps<const M: usize>(self) -> [Step; M] {
        debug_assert!(M == N);

        let scan_state = |last: &mut Option<Interval>, item: Interval| -> Option<Step> {
            let step = last.map_or(Step::from(item), |prev| Step::from(item - prev));
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

impl<const N: usize> IntoPitches for [Interval; N] {
    fn into_pitches<const M: usize>(self, root: Pitch) -> [Pitch; M] {
        debug_assert!(M == N + 1);

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

#[cfg(test)]
mod tests {
    use super::*;
    
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
        assert_eq!(pitches, [Pitch::new(60), Pitch::new(62), Pitch::new(65), Pitch::new(69)]);
    }
}
