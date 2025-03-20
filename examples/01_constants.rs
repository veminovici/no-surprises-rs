use no_surprises::prelude::*;

fn main() {
    // Using predefined pitch constants
    println!("Predefined pitches:");
    println!("  C4 = {} semitones", C4.semitones());
    println!("  D4 = {} semitones", D4.semitones());
    println!("  E4 = {} semitones", E4.semitones());
    println!("  F4 = {} semitones", F4.semitones());
    println!("  G4 = {} semitones", G4.semitones());
    println!("  A4 = {} semitones", A4.semitones());
    println!("  B4 = {} semitones", B4.semitones());

    // Using predefined interval constants
    println!("\nPredefined intervals:");
    println!(
        "  Perfect Unison = {} semitones",
        PERFECT_UNISON.semitones()
    );
    println!("  Major Second = {} semitones", MAJOR_SECOND.semitones());
    println!("  Major Third = {} semitones", MAJOR_THIRD.semitones());
    println!(
        "  Perfect Fourth = {} semitones",
        PERFECT_FOURTH.semitones()
    );
    println!("  Perfect Fifth = {} semitones", PERFECT_FIFTH.semitones());
    println!("  Major Sixth = {} semitones", MAJOR_SIXTH.semitones());
    println!("  Major Seventh = {} semitones", MAJOR_SEVENTH.semitones());
    println!(
        "  Perfect Octave = {} semitones",
        PERFECT_OCTAVE.semitones()
    );

    // Using predefined step constants
    println!("\nPredefined steps:");
    println!("  Unison = {} semitones", UNISON.semitones());
    println!("  Half Step = {} semitones", HALF.semitones());
    println!("  Whole Step = {} semitones", WHOLE.semitones());
    println!(
        "  Whole and Half Step = {} semitones",
        WHOLE_AND_HALF.semitones()
    );
}
