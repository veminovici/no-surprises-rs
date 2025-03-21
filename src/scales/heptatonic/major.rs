use crate::prelude::*;
use paste::paste;

define_scale!(Major, super::constants::MAJOR_SCALE_STEPS);

pub use major::*;

pub mod constants {
    use super::*;

    /// Common scale patterns
    pub const MAJOR_SCALE_STEPS: [Step; 7] = [
        Step::new(2), // WHOLE
        Step::new(2), // WHOLE
        Step::new(1), // HALF
        Step::new(2), // WHOLE
        Step::new(2), // WHOLE
        Step::new(2), // WHOLE
        Step::new(1), // HALF
    ];

    // use crate::scales::constants::MAJOR_SCALE_STEPS;
}
