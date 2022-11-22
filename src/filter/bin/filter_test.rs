use rtaudio_lib::effect::Effect;
use rtaudio_lib::read_data;
use rtaudio_lib::write_data;
use std::env::args;
mod filter;
use filter::FIRFilter;

fn main() {
    let mut argit = args();
    let filter_form = argit.nth(1).clone();
    let cutoff_freq1 = argit.next().clone();
    let cutoff_freq2 = argit.next().clone();

    let filter_form: String = if let Some(filter_form) = filter_form {
        filter_form
    } else {
        panic!("No filter_form argument given")
    };

    let cutoff_freq1 = if let Some(cutoff_freq1) = cutoff_freq1 {
        if let Ok(cutoff_freq1) = str::parse::<f32>(&cutoff_freq1) {
            cutoff_freq1
        } else {
            panic!("cutoff1 frequency isn't given as unsigned int value");
        }
    } else {
        panic!("No cutoff1 frequency argument given");
    };

    let cutoff_freq2 = if let Some(cutoff_freq2) = cutoff_freq2 {
        if let Ok(cutoff_freq2) = str::parse::<f32>(&cutoff_freq2) {
            cutoff_freq2
        } else {
            panic!("cutoff frequency2 isn't given as unsigned int value");
        }
    } else {
        panic!("No cutoff frequency2 argument given");
    };

    let fsample_rate: f32 = 48000.0;
    let filter_len: usize = 512;
    let frame_size: usize = 1024;

    let mut my_fir_filter = if filter_form.eq("lp") {
        FIRFilter::low_pass(cutoff_freq1, filter_len, frame_size)
    } else if filter_form.eq("bp") {
        FIRFilter::band_pass(cutoff_freq1, cutoff_freq2, filter_len, frame_size)
    } else if filter_form.eq("hp") {
        FIRFilter::high_pass(cutoff_freq1, filter_len, frame_size)
    } else {
        FIRFilter::new()
    };
    my_fir_filter.print_weights();
}
