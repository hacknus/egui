#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")] // hide console window on Windows in release

use eframe::egui;
use eframe::egui::plot::{Legend, Line, Plot, PlotPoints};

fn main() -> Result<(), eframe::Error> {
    // Log to stdout (if you run with `RUST_LOG=debug`).
    tracing_subscriber::fmt::init();

    let options = eframe::NativeOptions {
        initial_window_size: Some(egui::vec2(350.0, 400.0)),
        ..Default::default()
    };
    eframe::run_native(
        "My egui App with a plot",
        options,
        Box::new(|_cc| Box::new(MyApp::default())),
    )
}

struct MyApp {
    log_mode: bool,
}

impl Default for MyApp {
    fn default() -> Self {
        Self { log_mode: true }
    }
}

impl eframe::App for MyApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            let height = 200.0;
            let border_x = 11.0;
            let border_y = 18.0;
            let width = 300.0;

            ui.heading("My egui Application");

            ui.add_space(border_y); // add some whitespace in y direction

            ui.checkbox(&mut self.log_mode, "Log Mode");

            ui.add_space(border_y); // add some whitespace in y direction

            ui.horizontal(|ui| {
                ui.add_space(border_x); // add some whitespace in x direction

                let my_plot = Plot::new("My Plot")
                    .height(height)
                    .width(width)
                    .log_axes([false, self.log_mode])
                    .legend(Legend::default());

                let graph: Vec<[f64; 2]> = vec![
                    [0.0, 0.0],
                    [2.0, 2.0],
                    [4.0, 4.0],
                    [5.0, 5.0],
                    [7.0, 7.0],
                    [15.0, 15.0],
                    [30.0, 30.0],
                    [60.0, 60.0],
                    [100.0, 100.0],
                    [500.0, 500.0],
                ]; // dummy data

                let graph_log = make_log_data(&graph);

                if self.log_mode {
                    my_plot.show(ui, |plot_ui| {
                        plot_ui.line(Line::new(PlotPoints::from(graph_log)).name("curve"));
                    });
                } else {
                    my_plot.show(ui, |plot_ui| {
                        plot_ui.line(Line::new(PlotPoints::from(graph)).name("curve"));
                    });
                }
            });

            ui.add_space(border_y); // add some whitespace in y direction
        });
    }
}

fn make_log_data(graph: &Vec<[f64; 2]>) -> Vec<[f64; 2]> {
    graph
        .iter()
        .map(|v| {
            if v[1] != 0.0 {
                [v[0], v[1].log(10.0)]
            } else {
                [v[0], (v[1] + 0.01).log(10.0)] // offset, is 0.01 good?
            }
        })
        .collect()
}