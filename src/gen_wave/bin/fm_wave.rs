use crate::wave::Wave;
use rtaudio_lib::write_data;
use std::f32;

#[derive(Debug)]
pub struct FMWave {
    freq: u32,
    freq_mod: f32,
    num_samples: usize,
    values: Vec<f32>,
}

impl FMWave {
    pub fn new(freq: u32, num_samples: usize, freq_mod: f32) -> FMWave {
        let fsample_rate: f32 = 48000.0;
        let modulator_hub: f32 = 100.0;
        let modulator_freq: f32 = freq_mod;
        let modulator_index: f32 = if freq_mod == 0.0 {
            0.0
        } else {
            freq as f32 / modulator_freq
        };
        let ffreq = freq as f32;
        let shift =
            |t: f32, fmod: f32, fs: f32| -> f32 { (2.0 * f32::consts::PI * t * fmod / fs).cos() };
        let values_data = (0..num_samples)
            .map(|i| {
                ((2.0 * f32::consts::PI * (ffreq / fsample_rate) * (i as f32)
                    + modulator_index * shift(i as f32, freq_mod, fsample_rate))
                .sin())
            })
            .collect();
        return FMWave {
            freq: freq,
            freq_mod: freq_mod,
            num_samples: num_samples,
            values: values_data,
        };
    }
}
impl Wave for FMWave {
    fn print(&self) {
        write_data(&self.values, self.num_samples);
    }
}
