use eframe::egui;
use crate::controller::AppController;
use super::PanelComponent;

pub struct GraphPanel;

impl PanelComponent for GraphPanel {
    fn render(&mut self, ui: &mut egui::Ui, controller: &mut AppController) {
        let response = ui.allocate_response(
            egui::vec2(controller.config.graph_width, controller.config.graph_height),
            egui::Sense::hover()
        );
        
        ui.painter().rect_stroke(
            response.rect,
            1.0,
            egui::Stroke::new(1.0, egui::Color32::WHITE)
        );
        
        ui.painter().rect_filled(
            response.rect,
            0.0,
            egui::Color32::from_rgb(30, 30, 35)
        );
        
    }
}

impl GraphPanel {}