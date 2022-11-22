use egui_extras::RetainedImage;
use plotters::prelude::*;
use plotters::style::Color;
use plotters_bitmap::bitmap_pixel::RGBPixel;
use plotters_bitmap::BitMapBackend;
use std::error::Error;

const WIDTH: usize = 800;
const HEIGHT: usize = 600;

pub fn plot(values_data: &[f32], size: usize) -> Result<(), Box<dyn Error>> {
    let data: Vec<(f32, f32)> = (0..size)
        .map(|x| x as f32)
        .zip(values_data.iter().cloned())
        .collect();
    //    print!("{:?}", data);
    let mut buf = vec![0u8; WIDTH * HEIGHT * 3];
    let root = BitMapBackend::<RGBPixel>::with_buffer(&mut buf, (WIDTH as u32, HEIGHT as u32))
        .into_drawing_area();
    root.fill(&BLACK)?;

    let mut chart = ChartBuilder::on(&root)
        .caption("rtaudio_example", ("Arial", 50).into_font())
        .margin(5)
        .set_all_label_area_size(30)
        .build_cartesian_2d(0.0..(size as f32), -1.0f32..1.0f32)
        .unwrap();

    chart.configure_mesh().draw()?;
    chart.draw_series(LineSeries::new(data, &RED)).unwrap();
    // let image = RetainedImage::from_image_bytes("test", &buf)?;
    // ToDo: return image here, check for lifetime and 2nd borrow
    Ok(())
}
