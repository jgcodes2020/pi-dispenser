use eframe::{App, NativeOptions};
use egui::{Align, CentralPanel, Key, Layout, Ui, Vec2, ViewportBuilder};

pub struct Application {
    red_text: String,
    red_count: u64,
    grn_text: String,
    grn_count: u64,
}

impl Default for Application {
    fn default() -> Self {
        Self {
            red_text: Default::default(),
            red_count: Default::default(),
            grn_text: Default::default(),
            grn_count: Default::default(),
        }
    }
}

impl Application {
    fn counter(ui: &mut Ui, text: &mut String, count: &mut u64) {
        ui.allocate_ui_with_layout(
            Vec2::new(50.0, 150.0),
            Layout::top_down(Align::Center),
            |ui| {
                if ui.button("+1").clicked() {
                    *count += 1;
                }
                let field = ui.text_edit_singleline(text);
                if field.lost_focus() {
                    if let Ok(value) = text.parse::<u64>() {
                        *count = value;
                    }
                    *text = count.to_string();

                }
                if ui.button("-1").clicked() {
                    *count -= 1;
                }
            },
        );
    }
}

impl App for Application {
    fn update(&mut self, ctx: &egui::Context, frame: &mut eframe::Frame) {
        CentralPanel::default().show(ctx, |ui| {
            ui.allocate_ui_with_layout(ui.available_size(), Layout::top_down(Align::Center), |ui| {
                // ui.allocate_ui_with_layout(Vec2::new(), layout, add_contents)
            })
        });
    }
}

impl Application {
    pub fn run() {
        const APP_ID: &str = "io.github.jgcodes2020.dispenser";
        let opts = NativeOptions {
            viewport: ViewportBuilder::default()
                .with_resizable(false)
                .with_inner_size(Vec2::new(800.0, 400.0)),
            ..Default::default()
        };

        eframe::run_native(APP_ID, opts, Box::new(|_| Box::new(Self::default()))).unwrap();
    }
}
