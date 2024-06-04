use std::{thread, time::Duration};

use crate::pwm::PwmToneBuzzer;

pub mod rick;


const fn note2midi(name_str: &str) -> u32 {
    if !name_str.is_ascii() {
        panic!("note names must be ASCII");
    }

    let name = name_str.as_bytes();

    let mut idx: usize;

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

    // parse the octave manually because Rust const
    {
        let mut octave: u32 = 0;
        while idx < name.len() {
            octave = octave * 10 + match name[idx] {
                b'0'..=b'9' => (name[idx] - b'0') as u32,
                _ => panic!("note names must end with a number")
            };
            idx += 1;
        }

        midi += (octave + 1) * 12;
    }

    midi
}

#[inline(always)]
pub fn buzzer_play_array(buzzer: &mut PwmToneBuzzer, bpm: f64, data: &[(u32, f64)], cancel: &mut impl FnMut() -> bool) -> bool {
    let beat_ns: f64 = 60_000_000_000f64 / bpm;

    for i in 0..data.len() {
        let (note, len) = data[i];
        
        // if playing repeated note, stop a bit at the end to give pause before the next beat
        if i < (data.len() - 1) && data[i + 1].0 == note {
            buzzer.play_midi(note);
            if crate::park_exact(Duration::from_nanos((beat_ns * (len - 0.125)) as u64), cancel) {
                return true
            }
            buzzer.stop();
            if crate::park_exact(Duration::from_nanos((beat_ns * 0.125) as u64), cancel) {
                return true
            }
        }
        else {
            // otherwise just play the note in full
            buzzer.play_midi(note);
            if crate::park_exact(Duration::from_nanos((beat_ns * len) as u64), cancel) {
                return true
            }
        }
    }
    false
}