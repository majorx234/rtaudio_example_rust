use crate::stft::{WindowType, STFT};
use plotly::{
    common::ColorScalePalette,
    layout::update_menu::{ButtonBuilder, UpdateMenu, UpdateMenuType},
    HeatMap, Layout, Plot,
};

pub fn calc_stft(all_samples: &[f32], window_size: usize, step_size: usize) -> Vec<Vec<f32>> {
    let mut spectrogram: Vec<Vec<f32>> = Vec::new();

    // let's initialize our short-time fourier transform
    let window_type: WindowType = WindowType::Hanning;
    let mut stft = STFT::<f32>::new(window_type, window_size, step_size);
    let mut spectrogram_column: Vec<f32> = std::iter::repeat(0.).take(stft.output_size()).collect();
    // iterate over all the samples in chunks of 3000 samples.
    // in a real program you would probably read from something instead.
    for some_samples in (&all_samples[..]).chunks(3000) {
        // append the samples to the internal ringbuffer of the stft
        stft.append_samples(some_samples);
        // as long as there remain window_size samples in the internal
        // ringbuffer of the stft
        while stft.contains_enough_to_compute() {
            // compute one column of the stft by
            // taking the first window_size samples of the internal ringbuffer,
            // multiplying them with the window,
            // computing the fast fourier transform,
            // taking half of the symetric complex outputs,
            // computing the norm of the complex outputs and
            // taking the log10
            stft.compute_column(&mut spectrogram_column[..]);
            // here's where you would do something with the
            // spectrogram_column...
            // drop step_size samples from the internal ringbuffer of the stft
            // making a step of size step_size
            spectrogram.push(spectrogram_column.clone());
            stft.move_to_next_column();
        }
    }
    spectrogram
}

/// Display a heat map, with buttons to allow for toggling of different
/// colorscales.
/// * `spectrum` is a slice of slice
pub fn heat_map_with_modifiable_colorscale(spectrogram: Vec<Vec<f32>>) {
    type HeatMapType = HeatMap<f64, f64, Vec<f32>>;
    let trace = HeatMapType::new_z(spectrogram)
        .transpose(true)
        .color_scale(ColorScalePalette::Viridis.into());
    let mut plot = Plot::new();
    plot.add_trace(trace);

    let buttons = IntoIterator::into_iter([
        ("Viridis", ColorScalePalette::Viridis),
        ("Portland", ColorScalePalette::Portland),
        ("Blackbody", ColorScalePalette::Blackbody),
    ])
    .map(|(label, palette)| {
        ButtonBuilder::new()
            .label(label)
            .push_restyle(HeatMapType::modify_all_color_scale(palette.into()))
            .build()
    })
    .collect();

    plot.set_layout(Layout::new().update_menus(vec![UpdateMenu::new()
            .ty(UpdateMenuType::Buttons)
            .y(0.8)
            .buttons(buttons)]));

    plot.show();
}
