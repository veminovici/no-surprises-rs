#![allow(incomplete_features)]
#![allow(unused)]
#![feature(generic_const_exprs)]

pub mod core;
pub mod scales;

/// The prelude module re-exports commonly used types and constants.
///
/// This module provides a convenient way to import the most commonly used
/// elements from the library. It includes:
///
/// - All core types (`Pitch`, `Interval`, `Step`)
/// - All conversion traits (`IntoIntervals`, `IntoSteps`, `IntoPitches`)
/// - All predefined constants
/// - Common scale types and functions
///
/// # Examples
///
/// ```rust
/// use no_surprises::prelude::*;
/// use no_surprises::scales::major_scale;
///
/// // Now you can use all the core types and constants directly
/// let pitch = C4;
/// let interval = MAJOR_THIRD;
/// let step = HALF;
///
/// // And work with scales
/// let scale = major_scale(C4);
/// ```
pub mod prelude {
    #[allow(unused_imports)]
    pub use crate::core::*;

    #[allow(unused_imports)]
    pub use crate::core::constants::*;
}

#[allow(unused_imports)]
pub use prelude::*;
