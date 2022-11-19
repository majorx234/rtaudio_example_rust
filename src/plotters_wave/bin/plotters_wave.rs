use plotters::prelude::*;

pub fn plot(values_data: &Vec<f32>, size: usize) {
    let data: Vec<(f32, f32)> = (0..size)
        .map(|x| x as f32)
        .zip(values_data.iter().cloned())
        .collect();
    print!("{:?}", data);
}
