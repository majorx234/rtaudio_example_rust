use rtaudio_lib::effect::Effect;
use rtaudio_lib::read_data;
use rtaudio_lib::write_data;
use std::env::args;
mod delay;
use delay::Delay;

fn main() {
    let mut argit = args();
    let time_in_sec = argit.nth(1).clone();
    let feedback = argit.next().clone();

    let time_in_sec = if let Some(time_in_sec) = time_in_sec {
        if let Ok(time_in_sec) = str::parse::<f32>(&time_in_sec) {
            time_in_sec
        } else {
            panic!("frequency isn't given as unsigned int value");
        }
    } else {
        panic!("No frequency argument given");
    };

    let feedback = if let Some(feedback) = feedback {
        if let Ok(feedback) = str::parse::<f32>(&feedback) {
            feedback
        } else {
            panic!("feedback isn't given as float value");
        }
    } else {
        panic!("No feedback argument given");
    };

    let (num_samples, input_data) = read_data();
    let mut values_data: Vec<f32> = vec![0.0; num_samples];
    let mut my_delay = Delay::new();
    my_delay.set_delay(time_in_sec);
    my_delay.set_feedback(feedback);
    let mut start_index: usize = 0;
    while start_index + 1024 < num_samples {
        my_delay.process_samples(
            Some(&input_data[start_index..start_index + 1024]),
            None,
            Some(&mut values_data[start_index..start_index + 1024]),
            None,
        );
        start_index += 1024;
    }
    let rest: usize = num_samples - start_index;
    my_delay.set_frame_size(rest);
    my_delay.process_samples(
        Some(&input_data[start_index..start_index + rest]),
        None,
        Some(&mut values_data[start_index..start_index + rest]),
        None,
    );
    write_data(&values_data, num_samples);
}
