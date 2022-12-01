use rtaudio_lib::effect::Effect;
use rtaudio_lib::read_data;
use rtaudio_lib::spectrogram::{calc_stft, heat_map_with_modifiable_colorscale};
use rtaudio_lib::stft;
use rtaudio_lib::write_data;

fn main() {
    let (num_samples, input_data) = read_data();
    let spectrogram: Vec<Vec<f32>> = calc_stft(&input_data, num_samples, 1024, 512);
    write_data(&spectrogram[0], spectrogram[0].len());
}
