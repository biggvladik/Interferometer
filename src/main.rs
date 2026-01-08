// src/main.rs
mod model;
mod view;
mod widgets;
mod app;

use eframe::egui;
use app::FixedPanelsApp;

fn main() -> Result<(), eframe::Error> {
    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default()
            .with_inner_size([1000.0, 600.0])
            .with_title("Управление позиционером"),
        ..Default::default()
    };

    eframe::run_native(
        "Управление позиционером",
        options,
        Box::new(|_cc| Box::<FixedPanelsApp>::default()),
    )
}