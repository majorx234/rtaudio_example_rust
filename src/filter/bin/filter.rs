use dsp::window;
use rtaudio_lib::effect::Effect;
use std::f32::consts::PI;

#[derive(Debug, Clone, PartialEq)]
pub enum FilterType {
    LowPass(f32),
    BandPass(f32, f32),
    HighPass(f32),
    None,
}

#[derive(Debug, Clone)]
pub struct FIRFilter {
    pub bypassing: bool,
    weights: Vec<f32>,
    weights_original: Vec<f32>,
    buffer_l: Vec<f32>,
    buffer_r: Vec<f32>,
    in_buffer_l: Vec<f32>,
    in_buffer_r: Vec<f32>,
    in_buffer_l_i_idx: usize,
    in_buffer_l_o_idx: usize,
    in_buffer_r_i_idx: usize,
    in_buffer_r_o_idx: usize,
    gain: f32,
    filter_type: FilterType,
    sample_rate: f32,
    len: usize,
    frame_size: usize,
}

impl FIRFilter {
    pub fn low_pass(cutoff: f32, filter_len: usize, frame_size: usize, sample_rate: f32) -> Self {
        let mut new_lp_filter = FIRFilter::new();
        new_lp_filter.set_frame_size(frame_size);
        new_lp_filter.set_sample_rate(sample_rate);
        let len = if new_lp_filter.len % 2 == 0 {
            new_lp_filter.len + 1
        } else {
            new_lp_filter.len
        };

        let mut sinc: Vec<f32> = vec![0.0; len];
        new_lp_filter.weights = vec![0.0; len];
        new_lp_filter.buffer_l = vec![0.0; len];
        new_lp_filter.buffer_r = vec![0.0; len];
        if new_lp_filter.len > new_lp_filter.frame_size {
            new_lp_filter.in_buffer_l = vec![0.0; len - frame_size];
            new_lp_filter.in_buffer_r = vec![0.0; len - frame_size];
            new_lp_filter.in_buffer_l_i_idx = 0;
            new_lp_filter.in_buffer_l_o_idx = 0;
            new_lp_filter.in_buffer_r_i_idx = 0;
            new_lp_filter.in_buffer_r_o_idx = 0;
        }

        let angular_cutoff = (2.0 * PI * cutoff) / new_lp_filter.sample_rate;

        let middle = (len / 2) as isize; // should be odd
        let blackman_window = window::blackman(len);
        for i in -middle..=middle {
            sinc[(i + middle) as usize] = (angular_cutoff * i as f32).sin() / (PI * i as f32);
        }
        sinc[middle as usize] = 2.0 * cutoff / new_lp_filter.sample_rate;
        blackman_window.apply(&sinc, &mut new_lp_filter.weights);

        new_lp_filter
    }

    pub fn band_pass(
        low_cutoff: f32,
        high_cutoff: f32,
        filter_len: usize,
        frame_size: usize,
        sample_rate: f32,
    ) -> Self {
        let mut new_bp_filter = FIRFilter::new();
        new_bp_filter.set_frame_size(frame_size);
        new_bp_filter.set_sample_rate(sample_rate);
        let len = if new_bp_filter.len % 2 == 0 {
            new_bp_filter.len + 1
        } else {
            new_bp_filter.len
        };

        let mut sinc: Vec<f32> = vec![0.0; len];
        new_bp_filter.weights = vec![0.0; len];
        new_bp_filter.buffer_l = vec![0.0; len];
        new_bp_filter.buffer_r = vec![0.0; len];
        if new_bp_filter.len > new_bp_filter.frame_size {
            new_bp_filter.in_buffer_l = vec![0.0; len - frame_size];
            new_bp_filter.in_buffer_r = vec![0.0; len - frame_size];
            new_bp_filter.in_buffer_l_i_idx = 0;
            new_bp_filter.in_buffer_l_o_idx = 0;
            new_bp_filter.in_buffer_r_i_idx = 0;
            new_bp_filter.in_buffer_r_o_idx = 0;
        }

        let angular_low_cutoff = (2.0 * PI * low_cutoff) / new_bp_filter.sample_rate;
        let angular_high_cutoff = (2.0 * PI * high_cutoff) / new_bp_filter.sample_rate;

        let middle = (len / 2) as isize; // should be odd
        let blackman_window = window::blackman(len);
        for i in -middle..=middle {
            sinc[(i + middle) as usize] = ((angular_high_cutoff * i as f32).sin()
                - (angular_low_cutoff * i as f32).sin())
                / (PI * i as f32);
        }
        sinc[middle as usize] =
            1.0 - 2.0 * ((high_cutoff - low_cutoff) / new_bp_filter.sample_rate);
        blackman_window.apply(&sinc, &mut new_bp_filter.weights);

        new_bp_filter
    }

    pub fn high_pass(cutoff: f32, filter_len: usize, frame_size: usize, sample_rate: f32) -> Self {
        let mut new_hp_filter = FIRFilter::new();
        new_hp_filter.set_sample_rate(sample_rate);
        new_hp_filter.set_frame_size(frame_size);
        let len = if new_hp_filter.len % 2 == 0 {
            new_hp_filter.len + 1
        } else {
            new_hp_filter.len
        };

        let mut sinc: Vec<f32> = vec![0.0; len];
        new_hp_filter.weights = vec![0.0; len];
        new_hp_filter.buffer_l = vec![0.0; len];
        new_hp_filter.buffer_r = vec![0.0; len];
        if new_hp_filter.len > new_hp_filter.frame_size {
            new_hp_filter.in_buffer_l = vec![0.0; len - frame_size];
            new_hp_filter.in_buffer_r = vec![0.0; len - frame_size];
            new_hp_filter.in_buffer_l_i_idx = 0;
            new_hp_filter.in_buffer_l_o_idx = 0;
            new_hp_filter.in_buffer_r_i_idx = 0;
            new_hp_filter.in_buffer_r_o_idx = 0;
        }

        let angular_cutoff = (2.0 * PI * cutoff) / new_hp_filter.sample_rate;

        let middle = (len / 2) as isize; // should be odd
        let blackman_window = window::blackman(len);
        for i in -middle..=middle {
            sinc[(i + middle) as usize] = -(angular_cutoff * i as f32).sin() / (PI * i as f32);
        }
        sinc[middle as usize] = 1.0 - 2.0 * cutoff / new_hp_filter.sample_rate;
        blackman_window.apply(&sinc, &mut new_hp_filter.weights);

        new_hp_filter
    }

    pub fn print_weights(&self) {
        println!("{}", self.weights.len());
        for sample in &self.weights {
            println!("{}", sample);
        }
    }

    pub fn set_frame_size(&mut self, new_frame_size: usize) {
        self.frame_size = new_frame_size;
    }

    pub fn set_sample_rate(&mut self, new_sample_rate: f32) {
        self.sample_rate = new_sample_rate;
    }
}

impl Effect for FIRFilter {
    fn new() -> Self {
        FIRFilter {
            bypassing: false,
            weights: Vec::new(),
            weights_original: Vec::new(),
            buffer_l: Vec::new(),
            buffer_r: Vec::new(),
            in_buffer_l: Vec::new(),
            in_buffer_r: Vec::new(),
            in_buffer_l_i_idx: 0,
            in_buffer_l_o_idx: 0,
            in_buffer_r_i_idx: 0,
            in_buffer_r_o_idx: 0,

            gain: 1.0,
            filter_type: FilterType::None,
            sample_rate: 48000.0,
            len: 1024,
            frame_size: 2048,
        }
    }
    fn name(&self) -> &'static str {
        "FIRFilter"
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
        let convolution_fct = if self.frame_size > self.weights.len() {
            |input: &[f32],
             output: &mut [f32],
             buffer: &mut [f32],
             weights: &[f32],
             frame_size: usize| {
                assert!(input.len() == output.len());
                let w_len = weights.len();

                // convolution
                //in = prev, out = this
                for (buf_sample, output_sample) in buffer.iter().zip(output.iter_mut()) {
                    *output_sample += *buf_sample;
                }

                // in = this, out = this
                for input_idx in 0..(frame_size - w_len) {
                    let sample_in = input[input_idx];
                    if sample_in != 0.0 {
                        for (output_idx, weight) in
                            (input_idx..input_idx + w_len).zip(weights.iter())
                        {
                            output[output_idx] += sample_in * weight;
                        }
                    }
                }

                // in = this, out = this + next
                for s in buffer.iter_mut() {
                    // zero out inter-frame buffer.
                    *s = 0.0;
                }
                for input_idx in (frame_size - w_len)..frame_size {
                    let sample_in = input[input_idx];
                    if sample_in != 0.0 {
                        let mut idx = 0;

                        // first add into this frame
                        while idx < (frame_size - input_idx) {
                            output[input_idx + idx] += sample_in * weights[idx];
                            idx += 1;
                        }
                        // then add into the next frame with help of the inter-frame buffer
                        while idx < w_len {
                            buffer[idx - (frame_size - input_idx)] += sample_in * weights[idx];
                            idx += 1;
                        }
                    }
                }
            }
        } else {
            // frame_size < weights.len
            |input: &[f32],
             output: &mut [f32],
             buffer: &mut [f32],
             weights: &[f32],
             frame_size: usize| {}
        };

        if let Some(input_l) = input_l {
            if let Some(output_l) = output_l {
                convolution_fct(
                    input_l,
                    output_l,
                    &mut self.buffer_l,
                    &self.weights,
                    self.frame_size,
                );
            }
        }
        if let Some(input_r) = input_r {
            if let Some(output_r) = output_r {
                convolution_fct(
                    input_r,
                    output_r,
                    &mut self.buffer_r,
                    &self.weights,
                    self.frame_size,
                );
            }
        }
    }

    fn bypass(&mut self) {
        self.bypassing = !self.bypassing;
    }
}
