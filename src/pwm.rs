/*
pwm.rs
Language: Rust 1.78.0
Author: Jacky Guo
Date: Jun. 17, 2024
*/

//! Contains definitions for devices that run on the Pi's built-in PWM channels.

use rppal::pwm::{self, Pwm};

type PwmError = rppal::pwm::Error;


/// Converts a pin number to a PWM channel. This didn't have to be done, it
/// just makes the code easier to understand where I'm actually using it.
fn to_pwm_channel(pin: u8) -> pwm::Channel {
    match pin {
        18 => pwm::Channel::Pwm0,
        19 => pwm::Channel::Pwm1,
        _ => panic!("Specified pin cannot be configured for PWM"),
    }
}

/// A tone buzzer on a PWM channel (either pin 18 or 19).
pub struct PwmToneBuzzer {
    pwm: Pwm,
}

impl PwmToneBuzzer {
    /// Allocates a PWM channel for a tone buzzer on the desired pin.
    /// Note that only pins 18 and 19 support PWM; using any other pin results in a panic.
    /// If PWM is not set up, it will not return.
    pub fn new(pin: u8) -> Result<PwmToneBuzzer, PwmError> {
        // Setup PWM on the desired pin. This can fail, so we use the ?
        // operator to return an error if it occurs.
        let pwm = Pwm::with_frequency(
            to_pwm_channel(pin),   // PWM channel (0 = pin 18, 1 = pin 19)
            440.0,                 // frequency (changing affects pitch)
            0.5,                   // duty cycle (changing it affects timbre)
            pwm::Polarity::Normal, // polarity
            false,                 // enabled
        )?;

        Ok(Self { pwm })
    }

    /// Sets this tone buzzer to play a note. If the note is 0, stops the tone buzzer instead.
    pub fn play_midi(&mut self, midi: u32) {
        // Stop the tone buzzer if the note is 0.
        if midi == 0 {
            self.stop();
            return
        }
        // this optimized formula converts a MIDI note to a frequency.
        let freq = (midi as f64 / 12.0).exp2() * 8.175_798_915_643_707_33;
        // Set the frequency from above; the 2nd parameter is duty cycle.
        // This affects the timbre of the resulting note, I found 0.25 to be less harsh than 0.5.
        self.pwm.set_frequency(freq, 0.25).unwrap();
        // Enable the tone buzzer.
        self.pwm.enable().unwrap();
    }

    pub fn stop(&mut self) {
        // Disable the tone buzzer (stopping output).
        self.pwm.disable().unwrap();
    }
}
