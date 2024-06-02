use std::{thread, time::{Duration, Instant}};

use gui::Application;
use music::buzzer_play_array;
use pwm::PwmToneBuzzer;

mod gpio;
mod gui;
mod pwm;
mod music;

/// Parks the thread for a specified duration, unless a condition becomes true.
/// Returns true if interrupted, false otherwise.
pub(crate) fn park_exact(dur: Duration, cond: &mut impl FnMut() -> bool) -> bool {
    let expect_end = Instant::now() + dur;
    loop {
        let now = Instant::now();
        if now > expect_end {
            return false;
        } else {
            thread::park_timeout(expect_end - now);
        }
        if cond() {
            return true;
        }
    }
}

fn main() {
    let mut buzzer = PwmToneBuzzer::new(18).unwrap();
    buzzer_play_array(&mut buzzer, music::rick::BPM, &music::rick::DATA, &mut || false);
}
