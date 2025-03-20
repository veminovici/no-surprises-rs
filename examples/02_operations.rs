use no_surprises::prelude::*;

fn main() {
    // Create a pitch and step
    let c4 = Pitch::new(60);
    let whole_step = Step::new(2);

    // Add step to pitch
    let d4 = c4 + whole_step;
    println!("C4 + whole step = {} semitones", d4.semitones());

    // Create an interval from a step
    let major_second = Interval::from(whole_step);
    println!("Major second interval = {} semitones", major_second.semitones());

    // Create a step from an interval
    let step_from_interval = Step::from(major_second);
    println!("Step from major second = {} semitones", step_from_interval.semitones());

    // Create a sequence of steps
    let steps: [Step; 3] = [WHOLE, WHOLE, HALF];
    
    // Convert steps to intervals
    let intervals: [Interval; 3] = steps.into_intervals();
    println!("\nIntervals from steps:");
    for interval in intervals.iter() {
        println!("  {} semitones", interval.semitones());
    }

    // Convert steps to pitches
    let pitches: [Pitch; 4] = steps.into_pitches(c4);
    println!("\nPitches from steps:");
    for pitch in pitches.iter() {
        println!("  {} semitones", pitch.semitones());
    }

    // Calculate distance between pitches
    let distance = d4 - c4;
    println!("\nDistance between D4 and C4 = {} semitones", distance.semitones());
} 