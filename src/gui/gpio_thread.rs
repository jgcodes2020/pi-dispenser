use std::{sync::{atomic::{AtomicBool, Ordering}, Arc, Mutex}, thread, time::{Duration, Instant}};

use crate::{gpio::ServoSg90, park_exact};

use super::SharedState;



pub(crate) fn run_gpio_thread(state: Arc<SharedState>, egui_ctx: egui::Context) {
    let mut next_order: Option<(u64, u64)>;
    let mut cur_exit: bool;

    let mut servo_a = ServoSg90::new(17, 0.0).expect("Could not bind servo at pin 17");
    let mut servo_b = ServoSg90::new(27, 0.0).expect("Could not bind servo at pin 27");

    // main loop
    loop {
        // two things we're checking: whether we should exit or whether we have an order to run
        next_order = *state.next_order.lock().unwrap();
        cur_exit = state.exit_flag.load(Ordering::SeqCst);
        // wait for either of these things to change
        while let (None, false) = (next_order, cur_exit) {
            thread::park();
            next_order = *state.next_order.lock().unwrap();
            cur_exit = state.exit_flag.load(Ordering::SeqCst);
        }
        // if we're requested to exit the app, break
        if cur_exit {
            break;
        }

        // otherwise we must have an order, start processing it
        state.is_processing.store(true, Ordering::SeqCst);
        egui_ctx.request_repaint();

        let (red_count, green_count) = next_order.expect("We should have an order!");
        println!("ORDER: {}, {}", red_count, green_count);

        // execute the order. Any sleep must be replaced with a park (so that it can be interrupted)
        let mut wait_fn = || {
            cur_exit = state.exit_flag.load(Ordering::SeqCst);
            cur_exit
        };
        

        'exec: {
            // this is our equivalent of arduino delay, but it can be interrupted
            macro_rules! delay {
                ($dur:expr) => {
                    if park_exact(Duration::from_millis($dur), &mut wait_fn) {
                        break 'exec;
                    }
                };
            }

            for _ in 0..red_count {
                servo_a.set_pos(1.0);
                delay!(500);
                servo_a.set_pos(0.0);
                delay!(500);
            }
            for _ in 0..green_count {
                servo_b.set_pos(1.0);
                delay!(500);
                servo_b.set_pos(0.0);
                delay!(500);
            }
        }
        // reset the motors
        servo_a.set_pos(0.0);
        servo_b.set_pos(0.0);
        thread::sleep(Duration::from_millis(300));

        // signal that the order is over
        *state.next_order.lock().unwrap() = None;
        state.is_processing.store(false, Ordering::SeqCst);
        egui_ctx.request_repaint();
    }
}

