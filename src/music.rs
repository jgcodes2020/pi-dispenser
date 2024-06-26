/*
music.rs
Language: Rust 1.78.0
Author: Jacky Guo
Date: Jun. 17, 2024
*/

//! Contains utilities for programming and playing music 
//! on buzzers via the Pi's PWM channels.

use std::{time::Duration};

use crate::pwm::PwmToneBuzzer;
use crate::wait_interruptible;

pub mod rick;
pub mod badapple;

/// Converts a note name to its MIDI value.
/// 
/// ## Format
/// Note names must be written in the format:
/// ```text
/// <letter>[accidental]<octave>
/// ```
/// Note names are the capital letters A through G. Accidentals
/// are optional, and they can either be `#` (sharp) or `b` (flat).
/// Octave numbers must be non-negative; note that C4 = middle C.
/// 
/// As an example, some valid note names include:
/// ```text
/// C4
/// Bb3
/// F#6
/// ```
pub(crate) const fn note2midi(name_str: &str) -> u32 {
    if !name_str.is_ascii() {
        panic!("note names must be ASCII");
    }

    let name = name_str.as_bytes();

    let mut idx: usize;

    // Identify the base note name, and set its position according to the C major scale.
    let mut midi: u32 = match name[0] {
        b'C' => 0,
        b'D' => 2,
        b'E' => 4,
        b'F' => 5,
        b'G' => 7,
        b'A' => 9,
        b'B' => 11,
        _ => panic!("note names must start with a letter A-G")
    };
    // Add in any accidental (sharp '#' or flat 'b') if present
    match name[1] {
        b'0'..=b'9' => idx = 1,
        b'#' => {
            idx = 2;
            midi = match midi {
                0..=10 => midi + 1,
                11.. => midi - 11,
            }
        }
        b'b' => {
            idx = 2;
            midi = match midi {
                0 => midi + 11,
                1.. => midi - 1,
            }
        }
        _ => panic!("note names must have a number, optionally preceded by an accidental (# or b)")
    }

    // Treat the remaining digits as the octave number. Parsing needs to be done manually due to
    // ensure the function can run at compile-time.
    {
        let mut octave: u32 = 0;
        while idx < name.len() {
            octave = octave * 10 + match name[idx] {
                b'0'..=b'9' => (name[idx] - b'0') as u32,
                _ => panic!("note names must end with a number")
            };
            idx += 1;
        }

        // compute the final MIDI note by adding the correct number of semitones
        midi += (octave + 1) * 12;
    }

    midi
}

/// Plays music as defined by an array of pairs, each pair indicating note and duration.  
/// Returns true if the music was interrupted, or false if it played through to the end.
/// 
/// ## Parameters
/// 
/// - `buzzer``: the buzzer to play music on
/// - `bpm`: The tempo of the music, in BPM. 120 BPM corresponds to a beat every 0.5 seconds.
/// - `data`: A data array containing the actual music. See section "Data Format" for details.
/// - `cancel`: A function which can determine if an interrupt happens; allowing the music to be stopped whenever.
/// 
/// ## Data Format
/// Data is stored as a list of pairs. The first element of each pair is the MIDI index of the note being played,
/// and the second element is its length, in beats. As an example: `(60, 1.0)` is middle C played for the length
/// of a quarter note.
/// 
#[inline(always)]
pub fn buzzer_play_array(buzzer: &mut PwmToneBuzzer, bpm: f64, data: &[(u32, f64)], cancel: &impl Fn() -> bool) -> bool {
    // Compute the length of a beat in nanoseconds based on BPM.
    let beat_ns: f64 = 60_000_000_000f64 / bpm;

    // Macro for later: wait, if interrupted return true.
    macro_rules! delay {
        ($dur:expr) => {
            if wait_interruptible($dur, cancel) {
                return true;
            }
        };
    }

    for i in 0..data.len() {
        let (note, len) = data[i];
        
        // if playing a repeated note, stop a bit at the end to give pause before the next beat.
        // I chose the arbitrary duration of 1/8th of a beat, or a 32nd note. This is short enough to not be too obvious but not long enough for it to be obvious either.
        if i < (data.len() - 1) && data[i + 1].0 == note {
            buzzer.play_midi(note);
            delay!(Duration::from_nanos((beat_ns * (len - 0.125)) as u64));
            buzzer.stop();
            delay!(Duration::from_nanos((beat_ns * 0.125) as u64));
        }
        else {
            // otherwise just play the note for its full duration.
            buzzer.play_midi(note);
            delay!(Duration::from_nanos((beat_ns * len) as u64));
        }
    }
    false
}