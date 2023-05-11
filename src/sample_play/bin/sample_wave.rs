use hound::WavReader;
use rtaudio_lib::wave::Wave;
use rtaudio_lib::write_data;
use std::f32;
use std::path::Path;

#[derive(Debug)]
pub struct SampleWave {
    num_samples: usize,
    values: Vec<f32>,
}

impl SampleWave {
    pub fn new(filename: &String) -> SampleWave {
        let fsample_rate: f32 = 48000.0;
        let mut wave_reader = WavReader::open(Path::new(filename)).unwrap();
        let num_samples = wave_reader.len() as usize;
        let samples = wave_reader
            .samples::<i32>()
            .map(|x| x.unwrap() as f32 / 32768.0)
            .collect::<Vec<_>>();

        return SampleWave {
            num_samples: num_samples,
            values: samples,
        };
    }
}
impl Wave for SampleWave {
    fn print(&self) {
        write_data(&self.values);
    }
}
