use rtaudio_lib::effect::Effect;
use rtaudio_lib::read_data;
use rtaudio_lib::reverb::Reverb;
use rtaudio_lib::write_data;

fn main() {
    let (num_samples, input_data) = read_data();
    let mut values_data: Vec<f32> = vec![0.0; num_samples];
    let mut my_reverb = Reverb::new();
    my_reverb.process_samples(Some(&input_data), None, Some(&mut values_data), None);
    write_data(&values_data);
}
