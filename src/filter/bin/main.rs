extern crate scan_fmt;

use rtaudio_lib::effect::Effect;
use rtaudio_lib::read_data;
use rtaudio_lib::write_data;
mod filter;
use filter::FIRFilter;

fn main() {
    let (num_samples, input_data) = read_data();
    let mut values_data: Vec<f32> = Vec::with_capacity(num_samples);
    let mut my_fir_filter = FIRFilter::new();
    my_fir_filter.process_samples(Some(&input_data), None, Some(&mut values_data), None);
    write_data(&values_data, num_samples);
}
