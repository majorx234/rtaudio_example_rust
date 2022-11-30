#[macro_use]
use scan_fmt::scan_fmt;
pub mod delay;
pub mod effect;
pub mod simple_adsr;

pub fn read_data() -> (usize, std::vec::Vec<f32>) {
    let mut line = String::new();
    let line_size = std::io::stdin().read_line(&mut line).unwrap();
    let num_samples_raw = scan_fmt!(&line, "{}\n", usize);
    let num_samples: usize = num_samples_raw.unwrap();

    let mut values_data = Vec::new();
    for n in 0..num_samples {
        let mut line = String::new();
        let line_size = std::io::stdin().read_line(&mut line).unwrap();
        let sample = scan_fmt!(&line, "{}\n", f32);

        values_data.push(sample.unwrap());
    }
    return (num_samples, values_data);
}

pub fn write_data(values_data: &Vec<f32>, size: usize) {
    println!("{}", size);
    for sample in values_data {
        println!("{}", sample);
    }
}
