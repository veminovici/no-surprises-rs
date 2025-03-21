use crate::prelude::*;
use paste::paste;

define_scale!(NaturalMinor, super::constants::NATURALMINOR_SCALE_STEPS);

pub use naturalminor::*;

pub mod constants {
    use super::*;

    pub const NATURALMINOR_SCALE_STEPS: [Step; 7] = [
        Step::new(2), // WHOLE
        Step::new(1), // HALF
        Step::new(2), // WHOLE
        Step::new(2), // WHOLE
        Step::new(1), // HALF
        Step::new(2), // WHOLE
        Step::new(2), // WHOLE
    ];
}
