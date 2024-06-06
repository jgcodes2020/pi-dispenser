use std::{thread, time::{Duration, Instant}};

use gui::Application;
use music::buzzer_play_array;
use pwm::PwmToneBuzzer;

mod gpio;
mod gui;
mod pwm;
mod music;

// NOTE BELOW: In Rust, threads can be "parked", or put to sleep in a way that allows them to be interrupted.
// Interrupting a thread is done by calling Thread::unpark(). If a thread is unparked without already being parked, the next park will immediately end.

/// Parks the thread for a specified duration, waiting either for a timeout
/// or for an interrupt condition to be set.
pub(crate) fn wait_interruptible(dur: Duration, cond: &impl Fn() -> bool) -> bool {
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

/// Parks the thread for a specified duration, unless a condition becomes true.
/// Returns true if interrupted, false otherwise.
pub(crate) fn wait_pausable(dur: Duration, cond: &impl Fn() -> bool, pause_cond: &impl Fn() -> bool) -> bool {
    let expect_end = Instant::now() + dur;
    loop {
        let now = Instant::now();
        if pause_cond() {
            thread::park();
        }
        else {
            if now > expect_end {
                return false;
            } else {
                thread::park_timeout(expect_end - now);
            }
        }
        if cond() {
            return true;
        }
    }
}

fn main() {
    Application::run();
}
