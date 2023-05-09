use crate::wave::Wave;
use rtaudio_lib::write_data;
use std::f32;
use std::fs::File;
use std::path::Path;
use wav;

#[derive(Debug)]
pub struct SampleWave {
    num_samples: usize,
    values: Vec<f32>,
}

impl SampleWave {
    pub fn new(filename: &String) -> SampleWave {
        let num_samples: usize = 48000;
        let fsample_rate: f32 = 48000.0;
        let mut wave_file = File::open(Path::new(filename)).unwrap();
        let (header, data) = wav::read(&mut wave_file).unwrap();

        let values_data = vec![0.0; num_samples];
        return SampleWave {
            num_samples: num_samples,
            values: values_data,
        };
    }
}
impl Wave for SampleWave {
    fn print(&self) {
        write_data(&self.values);
    }
}
