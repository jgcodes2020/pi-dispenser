/*
gui.rs
Language: Rust 1.78.0
Author: Jacky Guo
Date: Jun. 17, 2024
*/

//! Implementation of the GUI for the app.


use std::{
    sync::{
        atomic::{AtomicBool, Ordering},
        Arc, Mutex,
    },
    thread::{self, JoinHandle},
};

use counter::{Counter, CounterState};
use eframe::{App, NativeOptions};
use egui::{Align, Button, CentralPanel, Layout, Vec2, ViewportBuilder};
use gpio_thread::run_gpio_thread;
use music_thread::run_music_thread;

mod counter;
mod gpio_thread;
mod music_thread;

/// Shared state between the various threads in the application.
#[derive(Default)]
struct SharedState {
    // Flags that signal exit, pause and cancel.
    exit_flag: AtomicBool,
    pause_flag: AtomicBool,
    cancel_flag: AtomicBool,
    // Flag set by the GPIO thread: true when an order is active.
    is_processing: AtomicBool,
    // The current order if one is being processed, or the next one if no order is being processed.
    next_order: Mutex<Option<(u64, u64)>>,
}

/// Primary state for the GUI.
pub struct Application {
    // Counter states for GUI.
    cnt_red: CounterState,
    cnt_green: CounterState,
    // Shared state between UI, GPIO and music threads.
    // Since the data isn't owned solely by the GUI, it needs to be reference-counted.
    shared_state: Arc<SharedState>,
    // Handles to the other threads so they can be cleaned up properly.
    gpio_join_handle: Option<JoinHandle<()>>,
    music_join_handle: Option<JoinHandle<()>>,
}

impl Application {
    /// Initializes the app. Requires an `egui` context to update the GUI from outside the GUI thread.
    pub fn new(egui_ctx: &egui::Context) -> Self {
        // Allocate the shared state on the heap; reference-counted to share between threads.
        let shared_state = Arc::<SharedState>::default();

        // Share the shared-state object and GUI handle to the two threads and start them.
        let gpio_thread = {
            let shared_state = Arc::clone(&shared_state);
            let egui_ctx = egui_ctx.clone();
            thread::spawn(move || run_gpio_thread(shared_state, egui_ctx))
        };
        let music_thread = {
            let shared_state = Arc::clone(&shared_state);
            let egui_ctx = egui_ctx.clone();
            thread::spawn(move || run_music_thread(shared_state, egui_ctx))
        };

        // Store all state in the Application struct
        Self {
            cnt_red: Default::default(),
            cnt_green: Default::default(),
            shared_state: shared_state,
            gpio_join_handle: Some(gpio_thread),
            music_join_handle: Some(music_thread),
        }
    }

    /// Starts an order based on the current GUI state.
    fn start_order(&self) {
        let mut order = self.shared_state.next_order.lock().unwrap();
        *order = Some((self.cnt_red.count(), self.cnt_green.count()));
        // Notify the GPIO thread that we can start.
        self.gpio_join_handle.as_ref().unwrap().thread().unpark();
    }
}

impl Drop for Application {
    // This function is run when Rust cleans up the application.
    fn drop(&mut self) {
        let gpio_join_handle = self.gpio_join_handle.take().unwrap();
        let gpio_thread = gpio_join_handle.thread();

        let music_join_handle = self.music_join_handle.take().unwrap();
        let music_thread = music_join_handle.thread();

        // set the exit flag
        self.shared_state.exit_flag.store(true, Ordering::SeqCst);

        // interrupt both background threads to let them know to exit
        gpio_thread.unpark();
        music_thread.unpark();

        // join both threads
        gpio_join_handle.join().expect("GPIO join failed!");
        music_join_handle.join().expect("Music join failed!")
    }
}

impl App for Application {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        CentralPanel::default().show(ctx, |ui| {
            let is_processing = self.shared_state.is_processing.load(Ordering::SeqCst);
            let is_paused = self.shared_state.pause_flag.load(Ordering::SeqCst);

            ui.allocate_ui_with_layout(
                ui.available_size(),
                Layout::top_down(Align::Center),
                |ui| {
                    // arrange the counters in a row
                    ui.allocate_ui_with_layout(
                        Vec2::new(200.0, 150.0),
                        Layout::left_to_right(Align::Center),
                        |ui| {
                            ui.add_enabled(
                                !is_processing,
                                Counter::new(&mut self.cnt_red).with_header("RED"),
                            );
                            ui.add_enabled(
                                !is_processing,
                                Counter::new(&mut self.cnt_green).with_header("GREEN"),
                            );
                        },
                    );
                    // start button
                    if ui
                        .add_enabled(
                            !is_processing,
                            Button::new("START").min_size(Vec2::new(150.0, 0.0)),
                        )
                        .clicked()
                    {
                        // start an order if one isn't already being processed
                        if !self.shared_state.is_processing.load(Ordering::SeqCst) {
                            self.start_order();
                        }
                    }
                    // pause/resume button
                    if ui
                        .add_enabled(
                            is_processing,
                            Button::new(if is_paused {"RESUME"} else {"PAUSE"}).min_size(Vec2::new(150.0, 0.0)),
                        )
                        .clicked()
                    {
                        // pause or resume the order if one is currently being processed.
                        if self.shared_state.is_processing.load(Ordering::SeqCst) {
                            self.shared_state.pause_flag.fetch_xor(true, Ordering::SeqCst);
                            self.gpio_join_handle.as_ref().unwrap().thread().unpark();
                        }
                    }
                    // stop immediately button
                    if ui
                        .add_enabled(
                            is_processing,
                            Button::new("CANCEL").min_size(Vec2::new(150.0, 0.0)),
                        )
                        .clicked()
                    {
                        // cancel the order if one is being processed
                        if self.shared_state.is_processing.load(Ordering::SeqCst) {
                            self.shared_state.cancel_flag.store(true, Ordering::SeqCst);
                            self.gpio_join_handle.as_ref().unwrap().thread().unpark();
                        }

                    }
                    // quit button
                    if ui
                        .add(Button::new("QUIT").min_size(Vec2::new(150.0, 0.0)))
                        .clicked()
                    {
                        // This just straight-up closes the window, which triggers the code in `Drop`
                        ctx.send_viewport_cmd(egui::ViewportCommand::Close);
                    }
                },
            );
        });
    }
}

impl Application {
    /// Runs the application GUI.
    pub fn run() {
        // This identifies the app on Wayland. I've used Java package naming to make it more unique.
        // If I install a .desktop file in ~/.local/share/applications named "io.github.jgcodes2020.dispenser.desktop", it would
        // use an icon from there. (Yes, Wayland doesn't simply let you set an icon because it likes to be special).
        const APP_ID: &str = "io.github.jgcodes2020.dispenser";

        // These are the options that are set on the display. For now, all this does is make sure it runs fullscreen.
        let opts = NativeOptions {
            viewport: ViewportBuilder::default()
                .with_resizable(false)
                .with_inner_size(Vec2::new(800.0, 480.0))
                .with_fullscreen(true)
                .with_title("POOTIS PENCER HERE"),

            ..Default::default()
        };

        // Run the app, passing the egui context to Application.
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
