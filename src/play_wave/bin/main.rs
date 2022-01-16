#[macro_use]
extern crate scan_fmt;
use rodio;
use rodio::buffer::SamplesBuffer;
use rodio::Sink;

fn read_data() -> (u32, std::vec::Vec<f32>) {
    let mut line = String::new();
    let line_size = std::io::stdin().read_line(&mut line).unwrap();
    let num_samples_raw = scan_fmt!(&line, "{}\n", u32);
    let num_samples: u32 = num_samples_raw.unwrap();

    let mut values_data = Vec::new();
    for n in 0..num_samples {
        let mut line = String::new();
        let line_size = std::io::stdin().read_line(&mut line).unwrap();
        let sample = scan_fmt!(&line, "{}\n", f32);

        values_data.push(sample.unwrap());
    }
    return (num_samples, values_data);
}

fn main() {
    let (num_samples, values_data) = read_data();

    let (_stream, handle) = rodio::OutputStream::try_default().unwrap();
    let sink = rodio::Sink::try_new(&handle).unwrap();
    let buffer = SamplesBuffer::new(1, 48000, values_data);
    sink.append(buffer);

    sink.sleep_until_end();
}
