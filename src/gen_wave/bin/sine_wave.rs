use rtaudio_lib::wave::Wave;
use rtaudio_lib::write_data;
use std::f32;

#[derive(Debug)]
pub struct SineWave {
    freq: u32,
    num_samples: usize,
    values: Vec<f32>,
}

impl SineWave {
    pub fn new(freq: u32, num_samples: usize) -> SineWave {
        let fsample_rate: f32 = 48000.0;
        let ffreq = freq as f32;

        let values_data = (0..num_samples)
            .map(|i| ((2.0 * f32::consts::PI * ffreq * (i as f32) / fsample_rate).sin()))
            .collect();
        return SineWave {
            freq: freq,
            num_samples: num_samples,
            values: values_data,
        };
    }
}
impl Wave for SineWave {
    fn print(&self) {
        write_data(&self.values);
    }
}
