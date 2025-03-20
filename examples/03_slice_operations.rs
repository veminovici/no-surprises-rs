use no_surprises::prelude::*;

fn main() {
    // Create some pitches
    let c4 = C4;
    let d4 = D4;
    let e4 = E4;
    let f4 = F4;

    // Create a sequence of pitches
    let pitches: [Pitch; 4] = [c4, d4, e4, f4];

    // Convert pitches to intervals
    let intervals: [Interval; 3] = pitches.into_intervals();
    println!("Intervals from C4 to F4:");
    for interval in intervals.iter() {
        println!("  {} semitones", interval.semitones());
    }

    // Convert pitches to steps
    let steps: [Step; 3] = pitches.into_steps();
    println!("\nSteps between pitches:");
    for step in steps.iter() {
        println!("  {} semitones", step.semitones());
    }

    // Create intervals from steps
    let new_intervals: [Interval; 3] = steps.into_intervals();
    println!("\nIntervals from steps:");
    for interval in new_intervals.iter() {
        println!("  {} semitones", interval.semitones());
    }

    // Create pitches from intervals
    let new_pitches: [Pitch; 4] = intervals.into_pitches(c4);
    println!("\nPitches from intervals:");
    for pitch in new_pitches.iter() {
        println!("  {} semitones", pitch.semitones());
    }
}
