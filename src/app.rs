// src/app.rs
use eframe::egui;
use crate::model::PositionerModel;
use crate::view::PositionerView;

pub struct FixedPanelsApp {
    model: PositionerModel,
}

impl FixedPanelsApp {
    pub fn new() -> Self {
        Self {
            model: PositionerModel::new(),
        }
    }
    
    pub fn handle_input(&mut self, ctx: &egui::Context) {
        if ctx.input(|i| i.key_pressed(egui::Key::Enter)) {
            self.model.update_current_from_string();
            self.model.update_destination_from_string();
        }
        
        self.model.update_graph_data();
    }
}

impl Default for FixedPanelsApp {
    fn default() -> Self {
        Self::new()
    }
}

impl eframe::App for FixedPanelsApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        self.handle_input(ctx);
        
        egui::CentralPanel::default().show(ctx, |ui| {
            PositionerView::show(ui, &mut self.model);
        });
    }
}