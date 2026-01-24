use eframe::egui;
use crate::controller::AppController;
use super::PanelComponent;

pub struct RightPanel;

impl PanelComponent for RightPanel {
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
            let content_rect = response.rect.shrink(15.0);
            
            ui.allocate_ui_at_rect(content_rect, |ui| {
                ui.vertical_centered(|ui| {
                    ui.add_space(10.0);
                    
                    ui.horizontal(|ui| {
                        ui.label(
                            egui::RichText::new("Repeat")
                                .size(12.0)
                                .color(egui::Color32::from_rgb(60, 60, 65)) // Темно-серый
                        );
                        
                        ui.add_space(5.0);
                        
                        ui.add(
                            egui::TextEdit::singleline(&mut controller.model.repeat_value)
                                .desired_width(50.0)
                                .font(egui::TextStyle::Monospace)
                                .hint_text("1")
                        );
                    });
                    
                    ui.add_space(10.0);
                });
            });
        });
    }
}