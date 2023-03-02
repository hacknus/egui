#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")] // hide console window on Windows in release

use eframe::egui;
use eframe::egui::plot::{GridInput, GridMark, Legend, Line, Plot, PlotPoint, PlotPoints};
use std::f64::consts::E;
use std::ops::RangeInclusive;

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
        Self { log_mode: false }
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
                let log_mode = self.log_mode;
                let label_fmt = move |s: &str, val: &PlotPoint| {
                    if log_mode {
                        format!("{}\n{:4.2} ps\n{:4.2} a.u.", s, val.x, val.y.exp())
                    } else {
                        format!("{}\n{:4.2} ps\n{:4.2} a.u.", s, val.x, val.y)
                    }
                };
                let log_mode = self.log_mode;
                let s_fmt = move |y: f64, _range: &RangeInclusive<f64>| {
                    if log_mode {
                        format!("{:4.2} a.u.", y.exp())
                    } else {
                        format!("{:4.2} a.u.", y)
                    }
                };

                let my_plot = Plot::new("My Plot")
                    .height(height)
                    .width(width)
                    .y_grid_spacer(logarithmic_grid_spacer(10))
                    .y_axis_formatter(s_fmt)
                    .label_formatter(label_fmt)
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
                [v[0], v[1].log(E)]
            } else {
                [v[0], (v[1] + 0.01).log(E)] // offset, is 0.01 good?
            }
        })
        .collect()
}

fn step_sizes(input: GridInput) -> Vec<GridMark> {
    vec![
        // 100s
        GridMark {
            value: 1.0_f64.log(E),
            step_size: 100.0,
        },
        GridMark {
            value: 10.0_f64.log(E),
            step_size: 100.0,
        },
        GridMark {
            value: 100.0_f64.log(E),
            step_size: 100.0,
        },
        // 25s
        GridMark {
            value: 2.0_f64.log(E),
            step_size: 1.0,
        },
        GridMark {
            value: 3.0_f64.log(E),
            step_size: 1.0,
        },
        GridMark {
            value: 4.0_f64.log(E),
            step_size: 1.0,
        },
        GridMark {
            value: 5.0_f64.log(E),
            step_size: 1.0,
        },
        GridMark {
            value: 6.0_f64.log(E),
            step_size: 1.0,
        },
        GridMark {
            value: 7.0_f64.log(E),
            step_size: 1.0,
        },
        GridMark {
            value: 8.0_f64.log(E),
            step_size: 1.0,
        },
        GridMark {
            value: 9.0_f64.log(E),
            step_size: 1.0,
        },
    ]
}

type GridSpacerFn = dyn Fn(GridInput) -> Vec<GridMark>;

type GridSpacer = Box<GridSpacerFn>;

fn next_power(value: f64, base: f64) -> f64 {
    assert_ne!(value, 0.0); // can be negative (typical for Y axis)
    base.powi(value.abs().log(base).ceil() as i32)
}
/// Fill in all values between [min, max] which are a multiple of `step_size`
fn generate_marks(step_sizes: [f64; 3], bounds: (f64, f64)) -> Vec<GridMark> {
    let mut steps = vec![];
    fill_marks_between(&mut steps, step_sizes[0], bounds);
    fill_marks_between(&mut steps, step_sizes[1], bounds);
    fill_marks_between(&mut steps, step_sizes[2], bounds);
    steps
}

/// Fill in all values between [min, max] which are a multiple of `step_size`
fn fill_marks_between(out: &mut Vec<GridMark>, step_size: f64, (min, max): (f64, f64)) {
    assert!(max > min);
    let first = (min / step_size).ceil() as i64;
    let last = (max / step_size).ceil() as i64;

    let marks_iter = (first..last).map(|i| {
        let value = (i as f64) * step_size;
        GridMark { value, step_size }
    });
    out.extend(marks_iter);
}
pub fn logarithmic_grid_spacer(log_base: i64) -> GridSpacer {
    let log_base = log_base as f64;
    let step_sizes = move |input: GridInput| -> Vec<GridMark> {
        // The distance between two of the thinnest grid lines is "rounded" up
        // to the next-bigger power of base
        dbg!(input.bounds);

        let smallest_visible_unit = next_power(input.base_step_size, log_base);

        let step_sizes = [
            smallest_visible_unit,
            smallest_visible_unit * log_base,
            smallest_visible_unit * log_base * log_base,
        ];


        generate_marks(step_sizes, input.bounds)
    };

    Box::new(step_sizes)
}
