use eframe::egui;
use rtaudio_lib::read_data;
mod plotters_wave;
use plotters_wave::plot;

struct PlottersApp {}

impl eframe::App for PlottersApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.heading("PlottersWave");
            ui.horizontal(|ui| {
                ui.heading("PlottersWave");
                //  ui.add(egui::Image::new(
                //        self.image.texture_id(ctx),
                //        self.image.size_vec2(),
                //  ));
            })
        });
    }
}

fn main() {
    let (num_samples, values_data) = read_data();
    let buf = plot(&values_data, num_samples);

    let plotters_app = PlottersApp {};
    let options = eframe::NativeOptions::default();
    eframe::run_native(
        "PlottersWave App",
        options,
        Box::new(|_cc| Box::new(plotters_app)),
    );
}
