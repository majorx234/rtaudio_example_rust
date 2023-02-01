mod audiodata;
use crate::audiodata::Audiodata;
mod wave_plotter_gui;
use crate::wave_plotter_gui::WavePlotterGui;
use eframe;
use rtaudio_lib::read_data;

fn main() {
    let (num_samples, values_data) = read_data();
    let mut wave_plotter_gui_app = WavePlotterGui::new();
    let audiodata = wave_plotter_gui_app.audiodata.append(&values_data);

    let mut options = eframe::NativeOptions::default();
    let window_size: eframe::egui::Vec2 = eframe::egui::Vec2::new(800.0, 600.0);
    options.initial_window_size = Some(window_size);
    eframe::run_native(
        "plot_wave_egui",
        options,
        Box::new(|_cc| Box::new(wave_plotter_gui_app)),
    );
}
