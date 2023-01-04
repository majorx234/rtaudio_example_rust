use rtaudio_lib::read_data;
use rtaudio_lib::simple_adsr as adsr;
use rtaudio_lib::write_data;
use std::env::args;

fn main() {
    let mut argit = args();
    let attack = argit.nth(1).clone();
    let decay = argit.next().clone();
    let sustain = argit.next().clone();
    let release = argit.next().clone();

    let attack = if let Some(attack) = attack {
        if let Ok(attack) = str::parse::<f32>(&attack) {
            attack
        } else {
            panic!("attack isn't given as f32 value");
        }
    } else {
        panic!("No attack argument given");
    };

    let decay = if let Some(decay) = decay {
        if let Ok(decay) = str::parse::<f32>(&decay) {
            decay
        } else {
            panic!("decay isn't given as f32 value");
        }
    } else {
        panic!("No decay argument given");
    };

    let sustain = if let Some(sustain) = sustain {
        if let Ok(sustain) = str::parse::<f32>(&sustain) {
            sustain
        } else {
            panic!("sustain isn't given as f32 value");
        }
    } else {
        panic!("No sustain argument given");
    };

    let release = if let Some(release) = release {
        if let Ok(release) = str::parse::<f32>(&release) {
            release
        } else {
            panic!("release isn't given as f32 value");
        }
    } else {
        panic!("No release argument given");
    };

    let (size, values_data) = read_data();
    let adsr_modificator: Vec<f32> =
        adsr::generate_adsr_modificator(size, attack, decay, sustain, release);
    let values_data: Vec<f32> = adsr::adsr_multiplication(adsr_modificator, values_data, size);
    write_data(&values_data, size);
}
