use crate::wave::Wave;
use rtaudio_lib::write_data;
use std::f32;

#[derive(Debug)]
pub struct AMWave {
    freq: u32,
    freq_mod: f32,
    num_samples: usize,
    values: Vec<f32>,
}

impl AMWave {
    pub fn new(freq: u32, num_samples: usize, freq_mod: f32) -> FMWave {
        let fsample_rate: f32 = 48000.0;
        let ffreq = freq as f32;
        let am_mod = |t: f32, fmod: f32, fs: f32| -> f32 {
            1.0 / fmod * (2.0 * f32::consts::PI * t * fmod / fs).sin()
        };
        let am_base = |t: f32, fbase: f32, fs: f32| -> f32 {
            (2.0 * f32::consts::PI * t * (fbase / fs)).sin()
        };
        let values_data = (0..num_samples)
            .map(|i| {
                (am_base(i as f32, ffreq, fsample_rate) * am_mod(i as f32, freq_mod, fsample_rate))
            })
            .collect();
        return AMWave {
            freq: freq,
            freq_mod: freq_mod,
            num_samples: num_samples,
            values: values_data,
        };
    }
}
impl Wave for AMWave {
    fn print(&self) {
        write_data(&self.values, self.num_samples);
    }
}
