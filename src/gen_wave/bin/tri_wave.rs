use rtaudio_lib::wave::Wave;
use rtaudio_lib::write_data;

#[derive(Debug)]
pub struct TriWave {
    freq: u32,
    num_samples: usize,
    values: Vec<f32>,
}

impl TriWave {
    pub fn new(freq: u32, num_samples: usize) -> TriWave {
        let fsample_rate: f32 = 48000.0;
        let ffreq = freq as f32;
        let fmax = fsample_rate / ffreq;
        let max: usize = fmax as usize;

        let shift: usize = max - (max / 4);

        let values_data = ((0 + shift)..(num_samples + shift))
            .map(|i| ((i % max) as f32 / fmax * 2.0 - 1.0).abs() * 2.0 - 1.0)
            .collect();

        return TriWave {
            freq: freq,
            num_samples: num_samples,
            values: values_data,
        };
    }
}

impl Wave for TriWave {
    fn print(&self) {
        write_data(&self.values);
    }
}
