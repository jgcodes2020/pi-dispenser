use egui::{Align, Button, Layout, TextEdit, Ui, Vec2, Widget};

pub struct CounterState {
    text: String,
    count: u64,
}

impl Default for CounterState {
    fn default() -> Self {
        Self {
            text: "0".to_owned(),
            count: 0,
        }
    }
}

impl CounterState {
    pub fn count(&self) -> u64 {
        self.count
    }

    pub fn set_count(&mut self, count: u64) {
        self.count = count;
        self.text = count.to_string();
    }
}

pub struct Counter<'a, 'b> {
    state: &'a mut CounterState,
    header: Option<&'b str>,
    enabled: bool,
}

impl<'a, 'b> Counter<'a, 'b> {
    pub fn new(state: &'a mut CounterState) -> Self {
        Self {
            state: state,
            header: None,
            enabled: true,
        }
    }

    pub fn with_enabled(self, enabled: bool) -> Self {
        Self {
            enabled: enabled,
            ..self
        }
    }

    pub fn with_header(self, header: &'b str) -> Self {
        Self {
            header: Some(header),
            ..self
        }
    }
}

impl<'a, 'b> Widget for Counter<'a, 'b> {
    fn ui(self, ui: &mut Ui) -> egui::Response {
        // Counter UI is stacked vertically
        ui.allocate_ui_with_layout(
            Vec2::new(100.0, 150.0),
            Layout::top_down(Align::Min),
            |ui| {
                let mut count_update = false;

                if let Some(header) = self.header {
                    ui.heading(header);
                }

                // +1 button
                if ui.add_enabled(self.enabled, Button::new("+1")).clicked() {
                    // increments the counter, checking against integer limit
                    // in practice we should never hit the integer limit
                    if self.state.count != u64::MAX {
                        self.state.count += 1;
                    }
                    count_update = true;
                }
                // text box containing the number
                if ui.add_enabled(self.enabled, TextEdit::singleline(&mut self.state.text)).lost_focus() {
                    // if the user types in a number, then mouses off, they can set it only if it's a valid number
                    if let Ok(value) = self.state.text.parse::<u64>() {
                        self.state.count = value;
                    }
                    count_update = true;
                }
                // -1 button
                if ui.add_enabled(self.enabled, Button::new("-1")).clicked() {
                    // decrements the counter, checking against 0
                    // 0 check is needed to prevent logic errors
                    if self.state.count != 0 {
                        self.state.count -= 1;
                    }
                    count_update = true;
                }

                // if the count was changed at all, update the text box
                if count_update {
                    self.state.text = self.state.count.to_string();
                }
            },
        )
        .response
    }
}
