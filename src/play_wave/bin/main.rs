#[macro_use]
extern crate scan_fmt;
use rodio;
use rodio::buffer::SamplesBuffer;
use rodio::Sink;
use rtaudio_lib::read_data;

fn main() {
    let (num_samples, values_data) = read_data();

    let (_stream, handle) = rodio::OutputStream::try_default().unwrap();
    let sink = rodio::Sink::try_new(&handle).unwrap();
    let buffer = SamplesBuffer::new(1, 48000, values_data);
    sink.append(buffer);

    sink.sleep_until_end();
}
