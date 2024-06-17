/*
gui/music_thread.rs
Language: Rust 1.78.0
Author: Jacky Guo
Date: Jun. 17, 2024
*/

//! Implementation of the music thread, which plays music on the buzzer.

use std::sync::{atomic::Ordering, Arc};

use crate::{music::buzzer_play_array, pwm::PwmToneBuzzer};

use crate::music::badapple as song;

use super::SharedState;

/// Function for the music thread, which plays music on the buzzer.
/// ## Parameters
/// - `state`: Shared state from the GUI.
/// - `_egui_ctx`: GUI context, also obtained from the GUI. Currently unused.
pub(super) fn run_music_thread(state: Arc<SharedState>, _egui_ctx: egui::Context) {
    let mut check_cur_exit = || state.exit_flag.load(Ordering::SeqCst);
    let mut buzzer = PwmToneBuzzer::new(18).unwrap();
    loop {
        if buzzer_play_array(&mut buzzer, song::BPM, &song::DATA, &mut check_cur_exit) {
            buzzer.stop();
            println!("STAHP!");
            break
        }
    }
}