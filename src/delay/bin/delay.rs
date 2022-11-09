use rtaudio_lib::effect::Effect;

pub struct Delay {
    pub bypassing: bool,
    delay_buffer_l: Vec<f32>,
    delay_buffer_r: Vec<f32>,
    delay_time: usize,
    feedback: f32,
    sample_rate: f32,
    frame_size: usize,
}

impl Delay {
    pub fn set_delay(&mut self, t_in_sec: f32) {
        let delay_time = self.sample_rate * t_in_sec;
        self.delay_time = self.delay_time.min(delay_time as usize);
    }

    pub fn set_feedback(&mut self, amount: f32) {
        self.feedback = amount.min(1.0);
    }
}

impl Effect for Delay {
    fn new() -> Self {
        let delay_buffer_size: usize = 48000 * 5;
        Delay {
            bypassing: false,
            delay_buffer_l: vec![0.0; delay_buffer_size],
            delay_buffer_r: vec![0.0; delay_buffer_size],
            delay_time: 48000,
            feedback: 0.33,
            sample_rate: 48000.0,
            frame_size: 1024,
        }
    }

    fn name(&self) -> &'static str {
        "Delay"
    }
    fn process_samples(
        &mut self,
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
