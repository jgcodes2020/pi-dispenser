/*
gpio.rs
Language: Rust 1.78.0
Author: Jacky Guo
Date: Jun. 17, 2024
*/

//! Classes that deal directly with the GPIO interface.

use rppal::{gpio::{Error, Gpio, OutputPin}, pwm::{self, Pwm}};
use std::{sync::OnceLock, time::Duration};

type GpioError = rppal::gpio::Error;

// In Rust, static variables cannot be directly modified. To make the internal value initializable exactly once, we use OnceLock.
static INSTANCE: OnceLock<Gpio> = OnceLock::new();

/// Obtains a base object needed to create other GPIO objects. This is a singleton used during the creation of all GPIO objects.
fn instance() -> &'static Gpio {
    INSTANCE.get_or_init(|| {
        match Gpio::new() {
            Ok(gpio) => gpio,
            Err(Error::PermissionDenied(msg)) => panic!("GPIO permission denied. You may need to add yourself to the GPIO group. msg:\n{}", msg),
            Err(err) => panic!("GPIO init error!\nmsg: {}", err)
        }
    })
}

/// Represents a Tower Pro SG90 Micro Servo on a GPIO pin. Uses software PWM to implement position, because
/// hardware PWM is limited to 2 channels; and it doesn't need to be too precise.
pub struct ServoSg90 {
    pin: OutputPin
}

impl ServoSg90 {
    // SG90 servos take a control pulse once every 20 ms.
    const PWM_PERIOD: Duration = Duration::from_micros(20_000);

    /// Converts a position from 0-1 to a pulse width. SG90 servos require a 1-2ms pulse (defined on datasheet); 
    /// this does restrict their range to 90 degrees.
    fn pulse_width(pos: f32) -> Duration {
        assert!(0.0 <= pos && pos <= 1.0);
        Duration::from_micros(((pos + 1.0) * 1.0e3) as u64)
    }

    /// Constructs a new servo, given a pin and initial position.
    /// The initial position must range from 0 to 1, if it is outside this range, this
    /// function panics.
    pub fn new(pin: u8, initial_pos: f32) -> Result<ServoSg90, GpioError> {
        let mut  pin = instance().get(pin)?.into_output_low();
        pin.set_pwm(Self::PWM_PERIOD, Self::pulse_width(initial_pos))?;

        Ok(Self { pin })
    }

    /// Moves the servo to the specified position. The position must be between 0 and 1.
    /// If it isn't, the function panics.
    pub fn set_pos(&mut self, pos: f32) {
        self.pin.set_pwm(Self::PWM_PERIOD, Self::pulse_width(pos)).unwrap();
    }
}

