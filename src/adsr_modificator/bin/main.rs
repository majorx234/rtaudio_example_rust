use rtaudio_lib::adsr;
use rtaudio_lib::read_data;
use rtaudio_lib::write_data;

fn main() {
    let (size, values_data) = read_data();
    let ta: f32 = 0.05;
    let td: f32 = 0.25;
    let ts: f32 = 0.6;
    let tr: f32 = 0.1;
    let adsr_modificator: Vec<f32> = adsr::generate_adsr_modificator(size, ta, td, ts, tr);
    let values_data: Vec<f32> = adsr::adsr_multiplication(adsr_modificator, values_data, size);
    write_data(&values_data, size);
}
