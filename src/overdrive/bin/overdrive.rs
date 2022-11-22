use rtaudio_lib::effect::Effect;

pub struct Overdrive {
    pub bypassing: bool,
    symetrical: bool,
}

impl Overdrive {
    pub fn set_symetrical(&mut self) {
        self.symetrical = true;
    }
    pub fn unset_symetrical(&mut self) {
        self.symetrical = false;
    }
}

impl Effect for Overdrive {
    fn new() -> Self {
        Overdrive {
            bypassing: false,
            symetrical: true,
        }
    }
    fn name(&self) -> &'static str {
        "overdrive"
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

        let symetrical_softclip = |x: f32| {
            let sign = x.signum();
            let x = x.abs();
            if 0.0 < x && x < 0.333 {
                sign * 2.0 * x
            } else if 0.333 < x && x < 0.666 {
                let t = 2.0 - 3.0 * x;
                sign * (3.0 - t * t) / 3.0
            } else {
                sign * 1.0
            }
        };

        let unsymetrical_softclip = |x: f32| {
            let x = x.abs();
            if 0.0 < x && x < 0.333 {
                2.0 * x
            } else if 0.333 < x && x < 0.666 {
                let t = 2.0 - 3.0 * x;
                (3.0 - t * t) / 3.0
            } else {
                1.0
            }
        };

        let softclip = if self.symetrical {
            symetrical_softclip
        } else {
            unsymetrical_softclip
        };

        let process_overdrive_on_slice = |input: Option<&[f32]>, output: Option<&mut [f32]>| {
            if let Some(input) = input {
                if let Some(output) = output {
                    for (index, xl) in input.iter().enumerate() {
                        output[index] = softclip(*xl);
                    }
                    if self.symetrical == false {
                        let average = output.iter().sum::<f32>() / (output.len() as f32);
                        output.iter_mut().for_each(|x| *x -= average);
                    }
                }
            }
        };

        process_overdrive_on_slice(input_l, output_l);
        process_overdrive_on_slice(input_r, output_r);
    }
    fn bypass(&mut self) {
        self.bypassing = !self.bypassing;
    }
}
