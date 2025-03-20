use no_surprises::prelude::*;
use no_surprises::scales::*;

#[derive(Debug)]
struct MyScaleQuality;
impl ScaleQuality for MyScaleQuality {
    /// The type alias for a major scale represented as steps
    type Steps = ScaleInSteps<Self, { Self::STEPS }>;

    /// The type alias for a major scale represented as intervals
    type Intervals = ScaleInIntervals<Self, { Self::INTERVALS }>;

    /// The type alias for a major scale represented as pitches
    type Pitches = ScaleInPitches<Self, { Self::PITCHES }>;

    /// The number of steps in a major scale (excluding the octave)
    const STEPS: usize = 3;

    /// The number of intervals in a major scale (excluding the octave)
    const INTERVALS: usize = Self::STEPS;

    /// The number of pitches in a major scale (including the octave)
    const PITCHES: usize = Self::STEPS + 1;
}

fn main() {
    let scale_in_steps = ScaleInSteps::<MyScaleQuality, 3>::new([WHOLE, HALF, HALF]);
    println!("Scale in steps: {:?}", scale_in_steps);

    let scale_in_intervals = scale_in_steps.to_scale_in_intervals::<3>();
    println!("Scale in intervals: {:?}", scale_in_intervals);

    let scale_in_pitches = scale_in_intervals.to_scale_in_pitches::<4>(C4);
    println!("Scale in pitches: {:?}", scale_in_pitches);
}
