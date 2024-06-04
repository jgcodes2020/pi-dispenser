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
use gpio_thread::run_gpio_thread;
use music_thread::run_music_thread;

use crate::{gpio::ServoSg90, park_exact};

mod counter;
mod gpio_thread;
mod music_thread;

#[derive(Default)]
struct SharedState {
    exit_flag: AtomicBool,
    is_processing: AtomicBool,
    next_order: Mutex<Option<(u64, u64)>>,
}

pub struct Application {
    // Counter states for GUI.
    cnt_red: CounterState,
    cnt_green: CounterState,
    // Shared state between UI and GPIO threads.
    shared_state: Arc<SharedState>,
    // Handle to the other threads so it can be cleaned up.
    gpio_join_handle: Option<JoinHandle<()>>,
    music_join_handle: Option<JoinHandle<()>>,
}

impl Application {
    pub fn new(egui_ctx: &egui::Context) -> Self {
        let shared_state = Arc::<SharedState>::default();
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

        Self {
            cnt_red: Default::default(),
            cnt_green: Default::default(),
            shared_state: shared_state,
            gpio_join_handle: Some(gpio_thread),
            music_join_handle: Some(music_thread)
        }
    }
}

impl Drop for Application {
    fn drop(&mut self) {
        let gpio_join_handle = self.gpio_join_handle.take().unwrap();
        let gpio_thread = gpio_join_handle.thread();
        
        let music_join_handle = self.music_join_handle.take().unwrap();
        let music_thread = music_join_handle.thread();

        // set the exit flag
        self.shared_state.exit_flag.store(true, Ordering::SeqCst);

        // interrupt both background threads
        gpio_thread.unpark();
        music_thread.unpark();
        
        // join both threads
        gpio_join_handle.join().expect("GPIO join failed!");
        music_join_handle.join().expect("Music join failed!")
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
                    // quit button
                    if ui.add(Button::new("QUIT").min_size(Vec2::new(150.0, 0.0))).clicked() {
                        ctx.send_viewport_cmd(egui::ViewportCommand::Close);
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
                .with_fullscreen(true)
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
