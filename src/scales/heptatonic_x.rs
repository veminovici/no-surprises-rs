use crate::prelude::*;
use paste::paste;

define_scale!(Major, [WHOLE, WHOLE, HALF, WHOLE, WHOLE, WHOLE, HALF]);
define_scale!(
    NaturalMinor,
    [WHOLE, HALF, WHOLE, WHOLE, HALF, WHOLE, WHOLE]
);

pub use major::constants::*;
pub use major::*;

pub use naturalminor::constants::*;
pub use naturalminor::*;

#[cfg(test)]
mod major_tests {
    use crate::prelude::*;
    use crate::scales::constants::*;
    use crate::scales::*;

    /// Test that the major scale step pattern matches the expected pattern
    #[test]
    fn test_major_scale_steps() {
        assert_eq!(
            MAJOR_SCALE_STEPS,
            [WHOLE, WHOLE, HALF, WHOLE, WHOLE, WHOLE, HALF]
        );
    }

    /// Test creating a major scale in steps
    #[test]
    fn test_major_scale_in_steps() {
        let scale = major_scale_in_steps();
        assert_eq!(
            scale.steps(),
            &[WHOLE, WHOLE, HALF, WHOLE, WHOLE, WHOLE, HALF]
        );
    }

    /// Test creating a major scale in intervals
    #[test]
    fn test_major_scale_in_intervals() {
        let scale = major_scale_in_intervals();
        assert_eq!(
            scale.intervals(),
            &[
                MAJOR_SECOND,
                MAJOR_THIRD,
                PERFECT_FOURTH,
                PERFECT_FIFTH,
                MAJOR_SIXTH,
                MAJOR_SEVENTH,
                PERFECT_OCTAVE
            ]
        );
    }

    /// Test creating a major scale in pitches from a root note
    #[test]
    fn test_major_scale_in_pitches() {
        let scale = major_scale_in_pitches(C4);
        assert_eq!(scale.pitches(), &[C4, D4, E4, F4, G4, A4, B4, C5]);
    }

    /// Test the convenience function for creating a major scale
    #[test]
    fn test_major_scale() {
        let scale = major_scale(C4);
        assert_eq!(scale.pitches(), &[C4, D4, E4, F4, G4, A4, B4, C5]);
    }

    /// Test converting between different scale representations
    #[test]
    fn test_major_conversions() {
        // Test pitch -> intervals -> pitch conversion
        let scale = major_scale(C4);
        let scale1 = scale.to_intervals();
        let scale2 = scale1.to_pitches();
        assert_eq!(scale, scale2);

        // Test pitch -> steps -> intervals -> pitch conversion
        let scale = major_scale(C4);
        let scale1 = scale.to_steps();
        let scale2 = scale1.to_intervals();
        let scale3 = scale2.to_pitches();
        assert_eq!(scale, scale3);
    }

    /// Test that the scale pattern is correct for different root notes
    #[test]
    fn test_major_scale_pattern_independence() {
        let c_scale = major_scale(C4);
        let g_scale = major_scale(G4);

        // The intervals between scale degrees should be the same
        let c_intervals = c_scale.to_intervals();
        let g_intervals = g_scale.to_intervals();
        assert_eq!(c_intervals.intervals(), g_intervals.intervals());

        // The steps should be the same
        let c_steps = c_scale.to_steps();
        let g_steps = g_scale.to_steps();
        assert_eq!(c_steps.steps(), g_steps.steps());
    }
}

#[cfg(test)]
mod naturalminor_tests {
    use super::*;
    // use super::naturalminor::*;
    // use super::naturalminor::constants::*;

    /// Test that the natural minor scale step pattern matches the expected pattern
    #[test]
    fn test_natural_minor_scale_steps() {
        assert_eq!(
            NATURALMINOR_SCALE_STEPS,
            [WHOLE, HALF, WHOLE, WHOLE, HALF, WHOLE, WHOLE]
        );
    }

    /// Test creating a natural minor scale in steps
    #[test]
    fn test_natural_minor_scale_in_steps() {
        let scale = naturalminor_scale_in_steps();
        assert_eq!(
            scale.steps(),
            &[WHOLE, HALF, WHOLE, WHOLE, HALF, WHOLE, WHOLE]
        );
    }

    /// Test creating a natural minor scale in intervals
    #[test]
    fn test_natural_minor_scale_in_intervals() {
        let scale = naturalminor_scale_in_intervals();
        assert_eq!(
            scale.intervals(),
            &[
                MAJOR_SECOND,
                MINOR_THIRD,
                PERFECT_FOURTH,
                PERFECT_FIFTH,
                MINOR_SIXTH,
                MINOR_SEVENTH,
                PERFECT_OCTAVE
            ]
        );
    }

    /// Test creating a natural minor scale in pitches from a root note
    #[test]
    fn test_natural_minor_scale_in_pitches() {
        let scale = naturalminor_scale_in_pitches(C4);
        assert_eq!(
            scale.pitches(),
            &[C4, D4, EFLAT4, F4, G4, AFLAT4, BFLAT4, C5]
        );
    }

    /// Test the convenience function for creating a natural minor scale
    #[test]
    fn test_natural_minor_scale() {
        let scale = naturalminor_scale(C4);
        assert_eq!(
            scale.pitches(),
            &[C4, D4, EFLAT4, F4, G4, AFLAT4, BFLAT4, C5]
        );
    }

    /// Test converting between different scale representations
    #[test]
    fn test_natural_minor_conversions() {
        // Test pitch -> intervals -> pitch conversion
        let scale = naturalminor_scale(C4);
        let scale1 = scale.to_intervals();
        let scale2 = scale1.to_pitches();
        assert_eq!(scale, scale2);

        // Test pitch -> steps -> intervals -> pitch conversion
        let scale = naturalminor_scale(C4);
        let scale1 = scale.to_steps();
        let scale2 = scale1.to_intervals();
        let scale3 = scale2.to_pitches();
        assert_eq!(scale, scale3);
    }

    /// Test that the scale pattern is correct for different root notes
    #[test]
    fn test_natural_minor_scale_pattern_independence() {
        let c_scale = naturalminor_scale(C4);
        let g_scale = naturalminor_scale(G4);

        // The intervals between scale degrees should be the same
        let c_intervals = c_scale.to_intervals();
        let g_intervals = g_scale.to_intervals();
        assert_eq!(c_intervals.intervals(), g_intervals.intervals());

        // The steps should be the same
        let c_steps = c_scale.to_steps();
        let g_steps = g_scale.to_steps();
        assert_eq!(c_steps.steps(), g_steps.steps());
    }
}
