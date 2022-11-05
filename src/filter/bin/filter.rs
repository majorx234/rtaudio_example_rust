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
    buffer: Vec<f32>,
    gain: f32,
    filter_type: FilterType,
    sample_rate: f32,
    len: usize,
}

impl FIRFilter {
    pub fn low_pass(cutoff: f32) -> Self {
        let mut new_lp_filter = FIRFilter::new();
        let len = if new_lp_filter.len % 2 == 0 {
            new_lp_filter.len + 1
        } else {
            new_lp_filter.len
        };

        let mut sinc: Vec<f32> = Vec::with_capacity(len);
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

    pub fn band_pass(low_cutoff: f32, high_cutoff: f32) -> Self {
        let mut new_bp_filter = FIRFilter::new();
        let len = if new_bp_filter.len % 2 == 0 {
            new_bp_filter.len + 1
        } else {
            new_bp_filter.len
        };

        let mut sinc: Vec<f32> = Vec::with_capacity(len);
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
}

impl Effect for FIRFilter {
    fn new() -> Self {
        FIRFilter {
            bypassing: false,
            weights: Vec::new(),
            weights_original: Vec::new(),
            buffer: Vec::new(),
            gain: 1.0,
            filter_type: FilterType::None,
            sample_rate: 48000.0,
            len: 1024,
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
