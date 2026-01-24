use eframe::egui;
use crate::controller::AppController;
use super::PanelComponent;

pub struct LeftPanel;

impl PanelComponent for LeftPanel {
    fn render(&mut self, ui: &mut egui::Ui, controller: &mut AppController) {
        let response = ui.allocate_response(
            egui::vec2(controller.config.panel_width, controller.config.panel_height),
            egui::Sense::hover()
        );
        
        ui.painter().rect_stroke(
            response.rect,
            1.0,
            egui::Stroke::new(0.8, egui::Color32::from_rgb(200, 200, 210)) // Светлая граница
        );
        
        ui.allocate_ui_at_rect(response.rect, |ui| {
            ui.vertical_centered(|ui| {
                let label_font_size = 12.0;
                let positioner_value_font_size = 14.0;
                let axis_value_font_size = 28.0;
                
                let positioner_width = 118.0;
                let positioner_height = 40.0;
                let axis_width = 118.0;
                let axis_height = 50.0;
                
                let button_width = 80.0;
                let button_height = 25.0;
                let button_font_size = 12.0;
                
                ui.label(
                    egui::RichText::new("Positioner")
                        .size(label_font_size)
                        .color(egui::Color32::from_rgb(60, 60, 65)) // Темно-серый
                );
                
                let pos_response = ui.allocate_response(
                    egui::vec2(positioner_width, positioner_height),
                    egui::Sense::hover()
                );
                
                ui.painter().rect_filled(
                    pos_response.rect,
                    0.0, 
                    egui::Color32::from_rgb(240, 240, 245) // Светлый фон
                );
                
                ui.painter().rect_stroke(
                    pos_response.rect,
                    0.0,
                    egui::Stroke::new(0.8, egui::Color32::from_rgb(200, 200, 210)) // Светлая граница
                );
                
                ui.painter().text(
                    pos_response.rect.center(),
                    egui::Align2::CENTER_CENTER,
                    &controller.model.positioner_name,
                    egui::FontId::proportional(positioner_value_font_size),
                    egui::Color32::from_rgb(60, 60, 65) // Темно-серый
                );
                
                ui.label(
                    egui::RichText::new("Axis name")
                        .size(label_font_size)
                        .color(egui::Color32::from_rgb(60, 60, 65)) // Темно-серый
                );
                
                let axis_response = ui.allocate_response(
                    egui::vec2(axis_width, axis_height),
                    egui::Sense::hover()
                );
                
                ui.painter().rect_filled(
                    axis_response.rect,
                    0.0,
                    egui::Color32::from_rgb(240, 240, 245) // Светлый фон
                );
                
                ui.painter().rect_stroke(
                    axis_response.rect,
                    0.0,
                    egui::Stroke::new(0.8, egui::Color32::from_rgb(200, 200, 210)) // Светлая граница
                );
                
                ui.painter().text(
                    axis_response.rect.center(),
                    egui::Align2::CENTER_CENTER,
                    &controller.model.axis_name,
                    egui::FontId::proportional(axis_value_font_size),
                    egui::Color32::from_rgb(60, 60, 65) // Темно-серый
                );
                
                ui.add_space(15.0);
                
                let button = egui::Button::new(
                    egui::RichText::new("Disconnect")
                        .size(button_font_size)
                        .color(egui::Color32::WHITE)
                ).min_size(egui::vec2(button_width, button_height))
                .fill(egui::Color32::from_rgb(200, 80, 80)); // Красный
                
                if ui.add(button).clicked() {
                    controller.handle_disconnect();
                }
            });
        });
    }
}