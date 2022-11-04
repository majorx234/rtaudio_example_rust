use rtaudio_lib::effect::Effect;

pub struct Overdrive {
    pub bypassing: bool,
}

impl Effect for Overdrive {
    fn new() -> Self {
        Overdrive { bypassing: false }
    }
    fn name(&self) -> &'static str {
        "overdrive"
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

        let symetrical_softclip = |x: f32| {
            if 0.0 < x && x < 0.333 {
                let t = 2.0 - 3.0 * x;
                (3.0 - t * t) / 3.0
            } else {
                x
            }
        };
        if let Some(input_l) = input_l {
            if let Some(output_l) = output_l {
                for (index, xl) in input_l.iter().enumerate() {
                    let xl = xl.abs();
                    output_l[index] = symetrical_softclip(xl);
                }
            }
        }
        if let Some(input_r) = input_r {
            if let Some(output_r) = output_r {
                for (index, xr) in input_r.iter().enumerate() {
                    let xr = xr.abs();
                    output_r[index] = symetrical_softclip(xr);
                }
            }
        }
    }
    fn bypass(&mut self) {
        self.bypassing = !self.bypassing;
    }
}