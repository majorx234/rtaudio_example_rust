mod wave_plotter;
use rtaudio_lib::read_data;
use wave_plotter::{plot, plot_advance};

fn main() {
    let (size, values_data) = read_data();
    plot(&values_data, size);
    plot_advance(&values_data, size);
}
