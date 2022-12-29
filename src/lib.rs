#[macro_use]
use scan_fmt::scan_fmt;
use std::io::BufRead;
pub mod delay;
pub mod effect;
pub mod overdrive;
pub mod simple_adsr;
pub mod spectrogram;
pub mod stft;

pub fn read_data() -> (usize, std::vec::Vec<f32>) {
    let mut line = String::new();
    let line_size = std::io::stdin().read_line(&mut line).unwrap();
    let num_samples_raw = scan_fmt!(&line, "{}\n", usize);
    let num_samples: usize = num_samples_raw.unwrap();

    let mut values_data = std::io::stdin()
        .lock()
        .lines()
        .map(|x| x.expect("0.0").parse::<f32>().unwrap())
        .collect::<Vec<f32>>();
    return (num_samples, values_data);
}

pub fn write_data(values_data: &Vec<f32>, size: usize) {
    println!("{}", size);
    for sample in values_data {
        println!("{}", sample);
    }
}
