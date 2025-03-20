pub mod constants;

mod interval;
mod pitch;
mod step;

pub use interval::*;
pub use pitch::*;
pub use step::*;

pub trait IntoIntervals {
    fn into_intervals<const M: usize>(self) -> [Interval; M];
}

pub trait IntoSteps {
    fn into_steps<const M: usize>(self) -> [Step; M];
}

pub trait IntoPitches {
    fn into_pitches<const M: usize>(self, root: Pitch) -> [Pitch; M];
}
