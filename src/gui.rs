use std::ops::RangeInclusive;

use device_query::{DeviceQuery, DeviceState};
use eframe::egui::{self, DragValue, Layout, ProgressBar, TextEdit, Widget};

pub struct App {
    device_state: DeviceState,
    play_key: String,
    iterations: String,
    delay: String,
    show_mouse_coords: bool,
    toggle: bool,
    use_expr_iteration: bool,
    dv_iteration: u32,
    dv_delay: u32,
}

const PADDING: f32 = 10.0;

#[derive(Default)]
pub struct State {}

impl Default for App {
    fn default() -> Self {
        Self {
            device_state: DeviceState::new(),
            show_mouse_coords: false,
            play_key: String::new(),
            iterations: String::new(),
            delay: String::new(),
            toggle: false,
            use_expr_iteration: false,
            dv_iteration: 0,
            dv_delay: 0,
        }
    }
}

impl eframe::App for App {
    fn update(&mut self, ctx: &eframe::egui::Context, frame: &mut eframe::Frame) {
        // egui::TopBottomPanel::top("top_bar").show(ctx, |ui| {
        //     ui.horizontal_wrapped(|ui| {
        //         ui.visuals_mut().button_frame = false;
        //
        //         egui::widgets::global_dark_light_mode_switch(ui);
        //
        //         ui.separator();
        //         ui.toggle_value(&mut self.toggle, "Backend");
        //         ui.separator();
        //     });
        // });

        // https://github.com/emilk/egui/blob/master/crates/egui_demo_lib/src/demo/widget_gallery.rs
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.horizontal_top(|ui| {
                egui::Grid::new("settings_grid")
                    .num_columns(2)
                    .striped(true)
                    .show(ui, |ui| {
                        if self.use_expr_iteration {
                            ui.label("Iterations");
                            // TODO: Disable interactive mode when not in idle state
                            ui.add(TextEdit::singleline(&mut self.iterations).interactive(true));
                            ui.end_row();
                        } else {
                            ui.label("Iterations");
                            // TODO: Disable interactive mode when not in idle state
                            ui.add(
                                DragValue::new(&mut self.dv_iteration)
                                    .clamp_range(RangeInclusive::new(0, 100000)),
                            );
                            ui.end_row();
                        }
                        // ui.label("Delay");
                        // // TODO: Disable interactive mode when not in idle state
                        // ui.add(TextEdit::singleline(&mut self.delay).interactive(true));
                        // ui.end_row();
                        //
                        ui.label("Delay (ms)");
                        // TODO: Disable interactive mode when not in idle state
                        ui.add(
                            DragValue::new(&mut self.dv_delay)
                                .clamp_range(RangeInclusive::new(0, 100000)),
                        );
                        ui.end_row();
                    });
                egui::Grid::new("keys_grid")
                    .num_columns(2)
                    .striped(true)
                    .show(ui, |ui| {
                        let play_label = ui.label("Play");
                        ui.button("key").labelled_by(play_label.id);
                        ui.end_row();

                        let record_label = ui.label("Record");
                        ui.button("key").labelled_by(record_label.id);
                        ui.end_row();
                    });

                ui.horizontal(|ui| {
                    let pause_label = ui.label("Pause");
                    ui.button("key").labelled_by(pause_label.id);
                });
            });
            ui.horizontal(|ui| {
                ui.checkbox(&mut self.use_expr_iteration, "Use expression for iteration");
            });

            ui.separator();

            ui.add_space(PADDING);
            ui.horizontal(|ui| {
                ui.heading("Total");
                ui.label("Total information");
            });
            ui.add(ProgressBar::new(0.4).show_percentage());

            ui.add_space(PADDING);
            ui.horizontal(|ui| {
                ui.heading("Total");
                ui.label("Total information");
            });
            ui.add(ProgressBar::new(0.4).show_percentage());
        });

        egui::TopBottomPanel::bottom("bottom_pannel")
            .resizable(false)
            .min_height(0.0)
            .show(ctx, |ui| {
                ui.horizontal(|ui| {
                    ui.checkbox(&mut self.show_mouse_coords, "Show mouse coordinates");
                    if self.show_mouse_coords {
                        ui.with_layout(Layout::right_to_left(eframe::emath::Align::Center), |ui| {
                            let (x, y) = self.device_state.get_mouse().coords;
                            ui.label(format!("  X: {}, Y: {}", x, y));
                            ctx.request_repaint();
                        });
                    }
                });
            });
    }
}

pub fn run() -> Result<(), eframe::Error> {
    println!("gui launched");
    let native_options = eframe::NativeOptions::default();
    eframe::run_native(
        "kare",
        native_options,
        Box::new(|_cc| Box::<App>::default()),
    )
}
