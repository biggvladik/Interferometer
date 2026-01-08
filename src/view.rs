// src/view.rs
use eframe::egui;
use crate::model::PositionerModel;
use crate::widgets::CustomWidgets;

pub struct PositionerView;

impl PositionerView {
    pub fn show(ui: &mut egui::Ui, model: &mut PositionerModel) {
        ui.vertical(|ui| {
            Self::show_control_panels(ui, model);
            ui.add_space(10.0);
            Self::show_graph_area(ui, model);
        });
    }
    
    fn show_control_panels(ui: &mut egui::Ui, model: &mut PositionerModel) {
        egui::ScrollArea::horizontal().show(ui, |ui| {
            ui.horizontal(|ui| {
                Self::draw_left_panel_frame(ui, model);
                ui.add_space(1.0);
                Self::draw_center_panels(ui, model);
                ui.add_space(5.0);
                Self::draw_right_panel_frame(ui, model);
            });
        });
    }
    
    fn draw_left_panel_frame(ui: &mut egui::Ui, model: &mut PositionerModel) {
        let left_response = ui.allocate_response(
            egui::vec2(model.panel_width, model.panel_height),
            egui::Sense::hover()
        );
        
        ui.painter().rect_stroke(
            left_response.rect,
            1.0,
            egui::Stroke::new(0.1, egui::Color32::WHITE)
        );
        
        ui.allocate_ui_at_rect(left_response.rect, |ui| {
            CustomWidgets::draw_left_panel(ui, model);
        });
    }
    
    fn draw_right_panel_frame(ui: &mut egui::Ui, model: &mut PositionerModel) {
        let right_response = ui.allocate_response(
            egui::vec2(model.panel_width, model.panel_height),
            egui::Sense::hover()
        );
        
        ui.painter().rect_stroke(
            right_response.rect,
            1.0,
            egui::Stroke::new(0.1, egui::Color32::WHITE)
        );
        
        ui.allocate_ui_at_rect(right_response.rect, |ui| {
            CustomWidgets::draw_right_panel(ui, model);
        });
    }
    
    fn draw_center_panels(ui: &mut egui::Ui, model: &mut PositionerModel) {
        ui.vertical(|ui| {
            Self::draw_top_sections(ui, model);
            ui.add_space(5.0);
            Self::draw_bottom_slider(ui, model);
        });
    }
    
    fn draw_top_sections(ui: &mut egui::Ui, model: &mut PositionerModel) {
        ui.horizontal(|ui| {
            Self::draw_section_frame(ui, model.section1_width, model.top_height, |ui| {
                CustomWidgets::draw_control_section1(ui, model);
            });
            
            Self::draw_section_frame_with_margin(ui, model.section2_width, model.top_height, |ui| {
                CustomWidgets::draw_control_section2(ui, model);
            });
            
            Self::draw_section_frame(ui, model.section3_width, model.top_height, |ui| {
                CustomWidgets::draw_control_section3(ui, model);
            });
        });
    }
    
    fn draw_bottom_slider(ui: &mut egui::Ui, model: &mut PositionerModel) {
        let bottom_response = ui.allocate_response(
            egui::vec2(model.bottom_width, model.bottom_height),
            egui::Sense::hover()
        );
        
        ui.painter().rect_stroke(
            bottom_response.rect,
            1.0,
            egui::Stroke::new(0.1, egui::Color32::WHITE)
        );
        
        ui.allocate_ui_at_rect(bottom_response.rect, |ui| {
            CustomWidgets::draw_slider(ui, model);
        });
    }
    
    fn draw_section_frame<F>(ui: &mut egui::Ui, width: f32, height: f32, content: F)
    where
        F: FnOnce(&mut egui::Ui),
    {
        let frame = egui::Frame::none()
            .inner_margin(4.0)
            .stroke(egui::Stroke::new(0.1, egui::Color32::WHITE));
        
        frame.show(ui, |ui| {
            ui.set_width(width);
            ui.set_height(height);
            content(ui);
        });
    }
    
    fn draw_section_frame_with_margin<F>(ui: &mut egui::Ui, width: f32, height: f32, content: F)
    where
        F: FnOnce(&mut egui::Ui),
    {
        let frame = egui::Frame::none()
            .inner_margin(egui::Margin {
                left: 15.0,
                right: 0.0,
                top: 8.0,
                bottom: 0.0,
            })
            .stroke(egui::Stroke::new(0.1, egui::Color32::WHITE));
        
        frame.show(ui, |ui| {
            ui.set_width(width);
            ui.set_height(height);
            content(ui);
        });
    }
    
    fn show_graph_area(ui: &mut egui::Ui, model: &PositionerModel) {
        let graph_width = 810.0;  // Фиксированная ширина
        let graph_response = ui.allocate_response(
            egui::vec2(graph_width, model.graph_height),
            egui::Sense::hover()
        );
        
        // Рисуем рамку области графика
        ui.painter().rect_stroke(
            graph_response.rect,
            1.0,
            egui::Stroke::new(1.0, egui::Color32::WHITE)
        );
        
        // Заполняем фон области графика
        ui.painter().rect_filled(
            graph_response.rect,
            0.0,
            egui::Color32::from_rgb(30, 30, 35)
        );
        
    }
}
