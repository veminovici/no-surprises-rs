use no_surprises::prelude::*;
use no_surprises::scales::*;

#[derive(Debug)]
struct MyScaleQuality;
impl ScaleQuality for MyScaleQuality {}

fn main() {
    let scale_in_steps = ScaleInSteps::<MyScaleQuality, 3>::new([WHOLE, HALF, HALF]);
    println!("Scale in steps: {:?}", scale_in_steps);

    let scale_in_intervals = scale_in_steps.to_scale_in_intervals::<3>();
    println!("Scale in intervals: {:?}", scale_in_intervals);

    let scale_in_pitches = scale_in_intervals.to_scale_in_pitches::<4>(C4);
    println!("Scale in pitches: {:?}", scale_in_pitches);
}
