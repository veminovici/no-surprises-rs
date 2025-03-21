use paste::paste;

define_scale!(
    NaturalMinor,
    [WHOLE, HALF, WHOLE, WHOLE, HALF, WHOLE, WHOLE]
);

pub use naturalminor::constants::*;
pub use naturalminor::*;

#[cfg(test)]
mod tests {
    use super::*;
    use crate::prelude::*;

    #[test]
    fn test_natural_minor_scale() {
        let scale = naturalminor_scale(C4);
        assert_eq!(scale.pitches(), &[C4, D4, EFLAT4, F4, G4, AFLAT4, BFLAT4, C5]);
    }
}
