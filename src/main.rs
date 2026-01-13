mod model;
mod controller;
mod view;

use eframe::egui;

fn main() -> Result<(), eframe::Error> {
    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default()
            .with_inner_size([1000.0, 600.0])
            .with_title("Управление подвижкой"),
        ..Default::default()
    };

    eframe::run_native(
        "Управление подвижкой",
        options,
        Box::new(|_cc| Box::<controller::AppController>::default()),
    )
}