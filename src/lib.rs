use std::io::BufRead;
pub mod delay;
pub mod effect;
pub mod overdrive;
pub mod simple_adsr;
pub mod spectrogram;
pub mod stft;
pub mod wave;

pub fn read_data() -> (usize, std::vec::Vec<f32>) {
    let values_data = std::io::stdin()
        .lock()
        .lines()
        .map(|x| x.expect("0.0").parse::<f32>().unwrap())
        .collect::<Vec<f32>>();
    let num_samples = values_data.len();
    return (num_samples, values_data);
}

pub fn write_data(values_data: &Vec<f32>) {
    for sample in values_data {
        println!("{}", sample);
    }
}
