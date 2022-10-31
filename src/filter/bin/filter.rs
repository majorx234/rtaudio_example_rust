use rtaudio_lib::effect::Effect;

#[derive(Debug, Clone)]
pub struct FIRFilter {
    pub bypassing: bool,
    weights: Vec<f32>,
    weights_original: Vec<f32>,
    buffer: Vec<f32>,
    gain: f32,
}

impl Effect for FIRFilter {
    fn new() -> Self {
        FIRFilter {
            bypassing: false,
            weights: Vec::new(),
            weights_original: Vec::new(),
            buffer: Vec::new(),
            gain: 1.0,
        }
    }
    fn name(&self) -> &'static str {
        "FIRFilter"
    }
    fn process_samples(
        &self,
        input_l: Option<&[f32]>,
        input_r: Option<&[f32]>,
        output_l: Option<&mut [f32]>,
        output_r: Option<&mut [f32]>,
    ) {
        if self.bypassing {
            if let Some(input_l) = input_l {
                if let Some(output_l) = output_l {
                    output_l.clone_from_slice(input_l);
                }
            }
            if let Some(input_r) = input_r {
                if let Some(output_r) = output_r {
                    output_r.clone_from_slice(input_r);
                }
            }
            return;
        }
    }
    fn bypass(&mut self) {
        self.bypassing = !self.bypassing;
    }
}
