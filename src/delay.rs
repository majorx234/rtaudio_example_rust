use crate::effect::Effect;

pub struct Delay {
    pub bypassing: bool,
    delay_buffer_l: Vec<f32>,
    delay_buffer_r: Vec<f32>,
    delay_buffer_l_i_idx: usize,
    delay_buffer_l_o_idx: usize,
    delay_buffer_r_i_idx: usize,
    delay_buffer_r_o_idx: usize,
    delay_time: usize,
    feedback: f32,
    sample_rate: f32,
    frame_size: usize,
}

impl Delay {
    pub fn set_delay(&mut self, t_in_sec: f32) {
        let delay_time = self.sample_rate * t_in_sec;
        self.delay_time = delay_time as usize;
    }

    pub fn set_feedback(&mut self, amount: f32) {
        self.feedback = amount.min(1.0);
    }
    pub fn set_frame_size(&mut self, new_frame_size: usize) {
        self.frame_size = new_frame_size;
    }
}

impl Effect for Delay {
    fn new() -> Self {
        let delay_buffer_size: usize = 48000 * 5;
        Delay {
            bypassing: false,
            delay_buffer_l: vec![0.0; delay_buffer_size],
            delay_buffer_r: vec![0.0; delay_buffer_size],
            delay_buffer_l_i_idx: 0,
            delay_buffer_l_o_idx: 0,
            delay_buffer_r_i_idx: 0,
            delay_buffer_r_o_idx: 0,
            delay_time: 5000,
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
        let delay_fct = |frame_size: usize,
                         input: &[f32],
                         output: &mut [f32],
                         delay_time: &usize,
                         feedback: f32,
                         buf_size: &usize,
                         buf: &mut [f32],
                         i_idx: &mut usize,
                         o_idx: &mut usize| {
            for index in 0..frame_size {
                if *i_idx >= *buf_size {
                    *i_idx = 0;
                }

                *o_idx = if *i_idx >= *delay_time {
                    *i_idx - *delay_time
                } else {
                    *buf_size + *i_idx - *delay_time
                };

                let y = input[index] + buf[index] * feedback;
                buf[*i_idx] = y;
                output[index] = y;
                *i_idx += 1;
            }
        };
        if let Some(input_l) = input_l {
            if let Some(output_l) = output_l {
                delay_fct(
                    self.frame_size,
                    input_l,
                    output_l,
                    &self.delay_time,
                    self.feedback,
                    &self.delay_buffer_l.len(),
                    &mut self.delay_buffer_l,
                    &mut self.delay_buffer_l_i_idx,
                    &mut self.delay_buffer_l_o_idx,
                );
            }
        }
        if let Some(input_r) = input_r {
            if let Some(output_r) = output_r {
                delay_fct(
                    self.frame_size,
                    input_r,
                    output_r,
                    &self.delay_time,
                    self.feedback,
                    &self.delay_buffer_r.len(),
                    &mut self.delay_buffer_r,
                    &mut self.delay_buffer_r_i_idx,
                    &mut self.delay_buffer_r_o_idx,
                );
            }
        }
    }

    fn bypass(&mut self) {
        self.bypassing = !self.bypassing;
    }
}
