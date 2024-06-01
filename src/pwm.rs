use std::time::Duration;

use rppal::pwm::{self, Pwm};

type PwmError = rppal::pwm::Error;

fn to_pwm_channel(pin: u8) -> pwm::Channel {
    match pin {
        18 => pwm::Channel::Pwm0,
        19 => pwm::Channel::Pwm1,
        _ => panic!("Specified pin cannot be configured for PWM"),
    }
}

fn note2period(note_str: &str) -> Duration {
    assert!(note_str.is_ascii());
    let note = note_str.as_bytes();
    let mut freq: f64 = match note[0usize] {
        b'C' => 32.7031956625748293,
        b'D' => 36.7080959896759450,
        b'E' => 41.2034446141087412,
        b'F' => 43.6535289291254856,
        b'G' => 48.9994294977186618,
        b'A' => 55.0000000000000000,
        b'B' => 61.7354126570188140,
        _ => panic!("Invalid note name {}", note_str),
    };

    let mut octave_idx = 2;
    match note[1] {
        b'#' => freq *= 1.05946309435929526,
        b'b' => freq /= 1.05946309435929526,
        b'0'..=b'9' => octave_idx = 1,
        _ => panic!("Invalid note name {}", note_str),
    }

    assert!(note.len() == octave_idx + 1);
    let octave: u64 = (note[octave_idx] - b'0') as u64;
    // bs bithack for multiplying a power of two unsafely
    let freq_bits = freq.to_bits();
    freq = f64::from_bits(freq_bits + (octave << 52));

    Duration::from_nanos((1e9 / freq) as u64)
}

pub struct PwmToneBuzzer {
    pwm: Pwm,
}

impl PwmToneBuzzer {
    pub fn new(pin: u8) -> Result<PwmToneBuzzer, PwmError> {
        let pwm = Pwm::with_period(
            to_pwm_channel(pin),
            note2period("A4"),
            note2period("A5"),
            pwm::Polarity::Normal,
            false,
        )
        .unwrap();

        Ok(Self { pwm })
    }

    pub fn play(&mut self, note: &str) {
        let dur = note2period(note);
        self.pwm.set_period(dur).unwrap();
        self.pwm.set_pulse_width(dur / 2).unwrap();
        self.pwm.enable().unwrap();
    }

    pub fn stop(&mut self) {
        self.pwm.disable().unwrap();
    }
}
