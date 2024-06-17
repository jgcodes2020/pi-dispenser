/*
main.rs
Language: Rust 1.78.0
Author: Jacky Guo
Date: Jun. 17, 2024
*/

//! Contains the main function, as well as a couple of nice functions for waiting
//! with the possibility of an interrupt.

use std::{thread, time::{Duration, Instant}};

use gui::Application;

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
    // let mut servo = ServoSg90::new(27, 0.0).unwrap();
    // let mut line: String = String::new();
    // let mut up = false;
    // loop {
    //     io::stdin().read_line(&mut line);
    //     up = !up;
    //     if up {
    //         servo.set_pos(1.0);
    //     }
    //     else {
    //         servo.set_pos(0.0);
    //     }
    // }

    Application::run();
}
