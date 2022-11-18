use plotly::{Plot, Scatter};

pub fn plot(values_data: &Vec<f32>, size: usize) {
    let time_data = (0..size).map(|x| x as f32).collect();
    let trace = Scatter::new(time_data, values_data.to_vec());
    let mut plot = Plot::new();
    plot.add_trace(trace);
    plot.show();
}
