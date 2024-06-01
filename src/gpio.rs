use rppal::{gpio::{Error, Gpio, OutputPin}, pwm::{self, Pwm}};
use std::{sync::OnceLock, time::Duration};

type GpioError = rppal::gpio::Error;

static INSTANCE: OnceLock<Gpio> = OnceLock::new();

pub fn instance() -> &'static Gpio {
    INSTANCE.get_or_init(|| {
        match Gpio::new() {
            Ok(gpio) => gpio,
            Err(Error::PermissionDenied(msg)) => panic!("GPIO permission denied. You may need to add yourself to the GPIO group. msg:\n{}", msg),
            Err(err) => panic!("GPIO init error!\nmsg: {}", err)
        }
    })
}

pub struct ServoSg90 {
    pin: OutputPin
}

impl ServoSg90 {
    const PWM_PERIOD: Duration = Duration::from_micros(20_000);

    fn pulse_width(pos: f32) -> Duration {
        assert!(0.0 <= pos && pos <= 1.0);
        Duration::from_micros(((pos + 1.0) * 1.0e3) as u64)
    }

    pub fn new(pin: u8, initial_pos: f32) -> Result<ServoSg90, GpioError> {
        let mut  pin = instance().get(pin)?.into_output_low();
        pin.set_pwm(Self::PWM_PERIOD, Self::pulse_width(initial_pos))?;

        Ok(Self { pin })
    }

    pub fn set_pos(&mut self, pos: f32) {
        self.pin.set_pwm(Self::PWM_PERIOD, Self::pulse_width(pos)).unwrap();
    }
}

