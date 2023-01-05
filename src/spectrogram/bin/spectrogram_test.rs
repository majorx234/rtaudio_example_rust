use rtaudio_lib::read_data;
use rtaudio_lib::spectrogram::calc_stft;
use rtaudio_lib::write_data;

fn main() {
    let (_, input_data) = read_data();
    let spectrogram: Vec<Vec<f32>> = calc_stft(&input_data, 1024, 512);
    write_data(&spectrogram[0], spectrogram[0].len());
}
