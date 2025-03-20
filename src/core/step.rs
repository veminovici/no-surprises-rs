use crate::UNISON;

use super::{Interval, IntoIntervals, IntoPitches, Pitch};

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub struct Step(u8);

impl Step {
    #[inline]
    pub const fn new(semitones: u8) -> Self {
        Self(semitones)
    }

    #[inline]
    pub const fn semitones(&self) -> u8 {
        self.0
    }
}

impl Default for Step {
    #[inline]
    fn default() -> Self {
        UNISON
    }
}

impl From<Interval> for Step {
    #[inline]
    fn from(interval: Interval) -> Self {
        Step(interval.semitones())
    }
}

impl<const N: usize> IntoIntervals for [Step; N] {
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

impl<const N: usize> IntoPitches for [Step; N] {
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

pub(crate) mod constants {
    use super::Step;
    pub const UNISON: Step = Step::new(0);
    pub const HALF: Step = Step::new(1);
    pub const WHOLE: Step = Step::new(2);
    pub const WHOLE_AND_HALF: Step = Step::new(3);

    pub const SEMITONE: Step = Step::new(1);
    pub const TONE: Step = Step::new(2);
}

#[cfg(test)]
mod tests {
    use super::*;

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
}
