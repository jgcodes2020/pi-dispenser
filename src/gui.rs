use std::{
    mem::MaybeUninit,
    sync::{
        atomic::{AtomicBool, Ordering},
        Arc, Condvar, Mutex, MutexGuard,
    },
    thread::{self, JoinHandle, Thread},
    time::{Duration, Instant},
};

use counter::{Counter, CounterState};
use eframe::{App, NativeOptions};
use egui::{Align, Button, CentralPanel, Key, Layout, Ui, Vec2, ViewportBuilder, Widget};

use crate::gpio::ServoSg90;

mod counter;

#[derive(Default)]
struct SharedState {
    exit_flag: AtomicBool,
    is_processing: AtomicBool,
    next_order: Mutex<Option<(u64, u64)>>,
}

fn run_gpio_thread(state: Arc<SharedState>, egui_ctx: egui::Context) {
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

/// Parks the thread for a specified duration, unless a condition becomes true.
/// Returns true if interrupted, false otherwise.
fn park_exact(dur: Duration, cond: &mut impl FnMut() -> bool) -> bool {
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

pub struct Application {
    // Counter states for GUI.
    cnt_red: CounterState,
    cnt_green: CounterState,
    // Shared state between UI and GPIO threads.
    shared_state: Arc<SharedState>,
    // Handle to the GPIO thread so it can be cleaned up.
    gpio_join_handle: Option<JoinHandle<()>>,
}

impl Application {
    pub fn new(egui_ctx: &egui::Context) -> Self {
        let shared_state = Arc::<SharedState>::default();
        let gpio_thread = {
            let shared_state = Arc::clone(&shared_state);
            let egui_ctx = egui_ctx.clone();
            thread::spawn(move || run_gpio_thread(shared_state, egui_ctx))
        };

        Self {
            cnt_red: Default::default(),
            cnt_green: Default::default(),
            shared_state: shared_state,
            gpio_join_handle: Some(gpio_thread),
        }
    }
}

impl Drop for Application {
    fn drop(&mut self) {
        let gpio_join_handle = self.gpio_join_handle.take().unwrap();
        let gpio_thread = gpio_join_handle.thread();

        // set the exit flag and interrupt
        self.shared_state.exit_flag.store(true, Ordering::SeqCst);
        gpio_thread.unpark();

        gpio_join_handle.join().expect("Thread join failed!");
    }
}

impl App for Application {
    fn update(&mut self, ctx: &egui::Context, frame: &mut eframe::Frame) {
        CentralPanel::default().show(ctx, |ui| {
            let can_enable = !self.shared_state.is_processing.load(Ordering::SeqCst);

            ui.allocate_ui_with_layout(
                ui.available_size(),
                Layout::top_down(Align::Center),
                |ui| {
                    // arrange counters in a row
                    ui.allocate_ui_with_layout(
                        Vec2::new(200.0, 150.0),
                        Layout::left_to_right(Align::Center),
                        |ui| {
                            ui.add(
                                Counter::new(&mut self.cnt_red)
                                    .with_header("RED")
                                    .with_enabled(can_enable),
                            );
                            ui.add(
                                Counter::new(&mut self.cnt_green)
                                    .with_header("GREEN")
                                    .with_enabled(can_enable),
                            );
                        },
                    );
                    // start button
                    if ui
                        .add_enabled(
                            can_enable,
                            Button::new("START").min_size(Vec2::new(150.0, 0.0)),
                        )
                        .clicked()
                    {
                        {
                            let mut order = self.shared_state.next_order.lock().unwrap();
                            *order = Some((self.cnt_red.count(), self.cnt_green.count()));
                        }
                        // notify the GPIO thread that we pressed Start
                        self.gpio_join_handle.as_ref().unwrap().thread().unpark();
                    }
                },
            );
        });
    }
}

impl Application {
    pub fn run() {
        const APP_ID: &str = "io.github.jgcodes2020.dispenser";
        let opts = NativeOptions {
            viewport: ViewportBuilder::default()
                .with_resizable(false)
                .with_inner_size(Vec2::new(800.0, 480.0))
                .with_maximized(true)
                .with_title("POOTIS PENCER HERE"),

            ..Default::default()
        };

        eframe::run_native(
            APP_ID,
            opts,
            Box::new(|ctx| {
                ctx.egui_ctx.set_zoom_factor(2.0);

                Box::new(Self::new(&ctx.egui_ctx))
            }),
        )
        .unwrap();
    }
}
