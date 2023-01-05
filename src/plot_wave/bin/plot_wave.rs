//use eframe::egui::{LayerId, Layout};
use plotly::common::Title;
use plotly::layout::{Axis, RangeSlider};
use plotly::{Layout, Plot, Scatter};

pub fn plot(values_data: &Vec<f32>, size: usize) {
    let time_data = (0..size).map(|x| x as f32).collect();
    let trace = Scatter::new(time_data, values_data.to_vec());
    let mut plot = Plot::new();
    plot.add_trace(trace);
    plot.show();
}

pub fn plot_advance(values_data: &Vec<f32>, size: usize) {
    let time_data = (0..size).map(|x| x as f32).collect();
    let trace = Scatter::new(time_data, values_data.to_vec());
    let mut plot = Plot::new();
    plot.add_trace(trace);

    let layout = Layout::new()
        .x_axis(Axis::new().range_slider(RangeSlider::new().visible(true)))
        .title(Title::new("plot advace"));
    plot.set_layout(layout);
    plot.show();
}
