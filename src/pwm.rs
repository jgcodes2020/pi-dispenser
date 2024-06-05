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

pub struct PwmToneBuzzer {
    pwm: Pwm,
}

impl PwmToneBuzzer {
    pub fn new(pin: u8) -> Result<PwmToneBuzzer, PwmError> {
        let pwm = Pwm::with_frequency(
            to_pwm_channel(pin),   // PWM channel (0 = pin 18, 1 = pin 19)
            440.0,                 // frequency (changing affects pitch)
            0.5,                   // duty cycle (changing it affects timbre)
            pwm::Polarity::Normal, // polarity
            false,                 // enabled
        )
        .unwrap();

        Ok(Self { pwm })
    }

    pub fn play_midi(&mut self, midi: u32) {
        if midi == 0 {
            self.stop();
            return
        }
        // this optimized formula converts a MIDI note to a frequency.
        let freq = (midi as f64 / 12.0).exp2() * 8.175_798_915_643_707_33;
        self.pwm.set_frequency(freq, 0.25).unwrap();
        self.pwm.enable().unwrap();
    }

    pub fn stop(&mut self) {
        self.pwm.disable().unwrap();
    }
}
