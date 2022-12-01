use rtaudio_lib::read_data;
mod plot_wave;
use plot_wave::plot_advance;

fn main() {
    let (num_samples, values_data) = read_data();
    plot_advance(&values_data, num_samples);
}
