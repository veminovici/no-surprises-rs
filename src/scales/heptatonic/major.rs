use paste::paste;

define_scale!(Major, [WHOLE, WHOLE, HALF, WHOLE, WHOLE, WHOLE, HALF]);

pub use major::constants::*;
pub use major::*;

#[cfg(test)]
mod tests {
    use super::*;
    use crate::prelude::*;

    #[test]
    fn test_major_scale() {
        let scale = major_scale(C4);
        assert_eq!(scale.pitches(), &[C4, D4, E4, F4, G4, A4, B4, C5]);
    }
}
