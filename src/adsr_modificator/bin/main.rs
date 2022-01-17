#[macro_use]
extern crate scan_fmt;

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

fn write_data(values_data: Vec<f32>, size: u32) {
    println!("{}", size);
    for sample in values_data {
        println!("{}", sample);
    }
}

fn generate_adsr_modificator(size: usize, ta: f32, td: f32, ts: f32, tr: f32) -> Vec<f32> {
    let mut values_data: Vec<f32> = Vec::with_capacity(size);
    let fmax_attack: f32 = ta * size as f32;
    let fmax_decay: f32 = td * size as f32;
    let fmax_sustain: f32 = ts * size as f32;
    let fmax_release: f32 = tr * size as f32;

    let max_attack: u32 = fmax_attack as u32;
    let max_decay: u32 = fmax_decay as u32;
    let max_sustain: u32 = fmax_sustain as u32;
    let max_release: u32 = max_sustain as u32;

    for n in 0..max_attack {
        let s: f32 = ((n % max_attack) as f32) / fmax_attack;
        values_data.push(s);
    }
    for n in max_attack..(max_attack + max_decay) {
        let j: u32 = n - max_attack;
        let s: f32 = 1.0 - (0.7 * ((j % max_decay) as f32) / fmax_decay);
        values_data.push(s);
    }
    for n in (max_attack + max_decay)..(max_attack + max_decay + max_sustain) {
        values_data.push(0.3);
    }
    for n in (max_attack + max_decay + max_sustain)..(size as u32) {
        let k: u32 = n - (max_attack + max_decay + max_sustain);
        let s: f32 = 0.3 - 0.3 * ((k % max_release) as f32) / fmax_release;
        values_data.push(s);
    }
    values_data
}

fn adsr_multiplication(in_audio: Vec<f32>, adsr_modificator: Vec<f32>, size: usize) -> Vec<f32> {
    let mut values_data: Vec<f32> = Vec::with_capacity(size);
    for n in 0..size {
        values_data.push(in_audio[n] * adsr_modificator[n]);
    }
    values_data
}

fn main() {
    let (size, values_data) = read_data();
    let ta: f32 = 0.05;
    let td: f32 = 0.25;
    let ts: f32 = 0.6;
    let tr: f32 = 0.1;
    let adsr_modificator: Vec<f32> = generate_adsr_modificator(size as usize, ta, td, ts, tr);
    let values_data: Vec<f32> = adsr_multiplication(adsr_modificator, values_data, size as usize);
    write_data(values_data, size);
}
