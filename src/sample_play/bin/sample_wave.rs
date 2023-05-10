use rtaudio_lib::wave::Wave;
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
        let fsample_rate: f32 = 48000.0;
        let mut wave_file = File::open(Path::new(filename)).unwrap();
        let (header, data) = wav::read(&mut wave_file).unwrap();

        //let values_data = vec![0.0; num_samples];
        let values_data = if let Some(values_data) = data.as_thirty_two_float() {
            values_data
        } else {
            return SampleWave {
                num_samples: 0,
                values: vec![],
            };
        };
        return SampleWave {
            num_samples: values_data.len(),
            values: (*values_data).to_vec(),
        };
    }
}
impl Wave for SampleWave {
    fn print(&self) {
        write_data(&self.values);
    }
}
