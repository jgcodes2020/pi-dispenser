use std::sync::{atomic::Ordering, Arc};

use crate::{music::*, pwm::PwmToneBuzzer};

use super::SharedState;


pub(super) fn run_music_thread(state: Arc<SharedState>, egui_ctx: egui::Context) {
    let mut check_cur_exit = || state.exit_flag.load(Ordering::SeqCst);
    let mut buzzer = PwmToneBuzzer::new(18).unwrap();
    loop {
        if buzzer_play_array(&mut buzzer, rick::BPM, &rick::DATA, &mut check_cur_exit) {
            buzzer.stop();
            println!("STAHP!");
            break
        }
    }
}