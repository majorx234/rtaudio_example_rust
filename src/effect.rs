pub trait Effect: Send {
    fn new() -> Self
    where
        Self: Sized;
    fn name(&self) -> &'static str;
    fn process_samples(
        &mut self,
        input_l: Option<&[f32]>,
        input_r: Option<&[f32]>,
        output_l: Option<&mut [f32]>,
        output_r: Option<&mut [f32]>,
    ) {
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
    }
    fn bypass(&mut self);
}
