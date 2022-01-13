use crate::wave::Wave;
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

        let mut values_data = Vec::new();

        for i in 0..num_samples {
            let s: f32 = 0.0;
            let n: usize = i % max;
            let f_n: f32 = n as f32;
            if f_n < (fmax * 0.5) {
                let s = ((2.0 * f_n) / fmax) - 0.5;
            } else {
                let s = ((-2.0 * f_n) / fmax) + 1.5;
            }
            let out: f32 = 2.0 * s;
            values_data.push(out);
        }

        return TriWave {
            freq: freq,
            num_samples: num_samples,
            values: values_data,
        };
    }
}

impl Wave for TriWave {
    fn print(&self) {
        println!("{}", self.num_samples);
        for sample in &self.values {
            println!("{}", sample);
        }
    }
}
