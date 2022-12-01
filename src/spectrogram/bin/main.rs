use rtaudio_lib::read_data;
use rtaudio_lib::spectrogram::{calc_stft, heat_map_with_modifiable_colorscale};
use rtaudio_lib::stft;
use std::env::args;

fn main() {
    let mut argit = args();
    let window_size = argit.nth(1).clone();
    let step_size = argit.next().clone();

    let window_size = if let Some(window_size) = window_size {
        if let Ok(window_size) = str::parse::<usize>(&window_size) {
            window_size
        } else {
            panic!("window_size isn't given as float value");
        }
    } else {
        panic!("No window_size argument given");
    };

    let step_size = if let Some(step_size) = step_size {
        if let Ok(step_size) = str::parse::<usize>(&step_size) {
            step_size
        } else {
            panic!("step_size isn't given as float value");
        }
    } else {
        panic!("No step_size argument given");
    };

    let (num_samples, input_data) = read_data();

    let spectrogram: Vec<Vec<f32>> = calc_stft(&input_data, num_samples, window_size, step_size);
    print!(
        "spec len: {}, spec[0].len: {}",
        spectrogram.len(),
        spectrogram[0].len()
    );
    heat_map_with_modifiable_colorscale(spectrogram);
}
