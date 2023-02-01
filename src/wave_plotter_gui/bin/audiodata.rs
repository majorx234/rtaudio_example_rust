use std::{collections::VecDeque, default};

pub struct Audiodata {
    pub values: VecDeque<f32>,
}

impl Audiodata {
    pub fn new() -> Self {
        Audiodata {
            values: VecDeque::new(),
        }
    }

    pub fn append(&mut self, samples: &[f32]) {
        for sample in samples {
            self.values.push_back(*sample);
        }
    }

    pub fn get_values(&self) -> Vec<f32> {
        self.values.iter().copied().collect::<Vec<f32>>()
    }
}
