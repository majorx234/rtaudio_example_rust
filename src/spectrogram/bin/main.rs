use rtaudio_lib::effect::Effect;
use rtaudio_lib::read_data;
mod spectrogram;
use spectrogram::{calc_stft, heat_map_with_modifiable_colorscale};
mod stft;

fn main() {
    let (num_samples, input_data) = read_data();
    let mut values_data: Vec<f32> = vec![0.0; num_samples];
    let spectrogram: Vec<Vec<f32>> = calc_stft(&input_data, num_samples);
    print!(
        "spec len: {}, spec[0].len: {}",
        spectrogram.len(),
        spectrogram[0].len()
    );
    heat_map_with_modifiable_colorscale(spectrogram);
}
