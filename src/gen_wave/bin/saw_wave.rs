use rtaudio_lib::wave::Wave;
use rtaudio_lib::write_data;

#[derive(Debug)]
pub struct SawWave {
    freq: u32,
    num_samples: usize,
    values: Vec<f32>,
}
impl SawWave {
    pub fn new(freq: u32, num_samples: usize) -> SawWave {
        let fsample_rate: f32 = 48000.0;
        let ffreq = freq as f32;
        let fmax = fsample_rate / ffreq;
        let max: usize = fmax as usize;

        let mut values_data = Vec::new();

        for n in 0..num_samples {
            let s_mod = n % max;
            let s: f32 = s_mod as f32;
            let out: f32 = 2.0 * (s / fmax) - 1.0;
            values_data.push(out);
        }

        return SawWave {
            freq: freq,
            num_samples: num_samples,
            values: values_data,
        };
    }
}

impl Wave for SawWave {
    fn print(&self) {
        write_data(&self.values);
    }
}
