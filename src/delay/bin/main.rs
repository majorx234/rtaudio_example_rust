use rtaudio_lib::effect::Effect;
use rtaudio_lib::read_data;
use rtaudio_lib::write_data;
mod delay;
use delay::Delay;

fn main() {
    let (num_samples, input_data) = read_data();
    let mut values_data: Vec<f32> = vec![0.0; num_samples];
    let mut my_delay = Delay::new();
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
