use rtaudio_lib::effect::Effect;
use rtaudio_lib::read_data;
mod spectrogram;
use spectrogram::heat_map_with_modifiable_colorscale;

fn main() {
    let (num_samples, input_data) = read_data();
    let mut values_data: Vec<f32> = vec![0.0; num_samples];
    heat_map_with_modifiable_colorscale();
}
