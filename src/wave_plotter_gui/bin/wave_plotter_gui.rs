use crate::audiodata::Audiodata;
use eframe::egui;
use std::sync::Arc;
use std::sync::Mutex;

use egui::plot::{
    Arrows, Bar, BarChart, CoordinatesFormatter, Corner, GridInput, GridMark, HLine, Legend, Line,
    LineStyle, MarkerShape, Plot, PlotImage, Points, Polygon, Text, VLine, Value, Values,
};

pub struct WavePlotterGui {
    pub audiodata: Audiodata,
}

impl WavePlotterGui {
    pub fn new() -> Self {
        Self {
            audiodata: Audiodata::new(),
        }
    }
}

impl Default for WavePlotterGui {
    fn default() -> Self {
        Self {
            audiodata: Audiodata::new(),
        }
    }
}

impl eframe::App for WavePlotterGui {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.heading("WavePlotterGui");
            let curve = self
                .audiodata
                .get_values()
                .into_iter()
                .enumerate()
                .map(|(x, y)| Value::new(x as f64, y as f64));
            //println!("{:?}", curve);
            let line = Line::new(Values::from_values_iter(curve));
            let plot = Plot::new("audio-signal")
                .include_y(-1.0)
                .include_y(1.0)
                .include_x(0.0)
                .show(ui, |plot_ui| plot_ui.line(line));
        });
        ctx.request_repaint();
    }
}
