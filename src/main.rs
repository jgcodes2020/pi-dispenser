use std::{thread, time::Duration};

use eframe::NativeOptions;
use egui::{Vec2, ViewportBuilder};
use gpio::ServoSg90;
use gui::Application;
use pwm::PwmToneBuzzer;

mod gpio;
mod gui;
mod pwm;

fn main() {
    Application::run();
}
