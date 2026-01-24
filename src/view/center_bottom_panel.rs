use eframe::egui;
use crate::controller::AppController;
use super::PanelComponent;

pub struct CenterBottomPanel;

impl PanelComponent for CenterBottomPanel {
    fn render(&mut self, ui: &mut egui::Ui, controller: &mut AppController) {
        let response = ui.allocate_response(
            egui::vec2(controller.config.bottom_width, controller.config.bottom_height),
            egui::Sense::hover()
        );
        
        // Светлая граница панели
        ui.painter().rect_stroke(
            response.rect,
            1.0,
            egui::Stroke::new(0.8, egui::Color32::from_rgb(200, 200, 210))
        );
        
        ui.allocate_ui_at_rect(response.rect, |ui| {
            ui.vertical_centered(|ui| {
                let slider_width = controller.config.bottom_width - 20.0;
                let slider_height = 40.0;
                
                ui.add_space(25.0);
                
                let slider_response = ui.allocate_response(
                    egui::vec2(slider_width, slider_height),
                    egui::Sense::click_and_drag()
                );
                
                let rect = slider_response.rect;
                
                self.render_slider_background(ui, rect);
                self.render_slider_track(ui, rect, controller);
                
                if slider_response.dragged() {
                    if let Some(pos) = ui.input(|i| i.pointer.interact_pos()) {
                        controller.handle_slider_drag(pos.x, rect.left(), rect.width());
                    }
                }
                
                ui.add_space(25.0);
            });
        });
    }
}

impl CenterBottomPanel {
    fn render_slider_background(&mut self, ui: &mut egui::Ui, rect: egui::Rect) {
        // Светлый фон слайдера
        ui.painter().rect_filled(
            rect,
            5.0,
            egui::Color32::from_rgb(245, 245, 250) // Очень светлый серо-голубой
        );
        
        // Небольшая тень для объема
        ui.painter().rect_stroke(
            rect,
            5.0,
            egui::Stroke::new(0.5, egui::Color32::from_rgb(220, 220, 225))
        );
    }
    
    fn render_slider_track(&mut self, ui: &mut egui::Ui, rect: egui::Rect, controller: &mut AppController) {
        let value_to_x = |value: f32| -> f32 {
            let normalized = (value - controller.model.slider_min) / (controller.model.slider_max - controller.model.slider_min);
            rect.left() + normalized * rect.width()
        };
        
        let current_x = value_to_x(controller.model.slider_current);
        let dest_x = value_to_x(controller.model.slider_destination);
        
        // Линия между ползунками - яркий синий
        ui.painter().line_segment(
            [egui::pos2(current_x, rect.center().y), 
             egui::pos2(dest_x, rect.center().y)],
            egui::Stroke::new(3.0, egui::Color32::from_rgb(0, 120, 215)) // Windows синий
        );
        
        // Основная линия шкалы - серый
        ui.painter().line_segment(
            [egui::pos2(rect.left(), rect.center().y), 
             egui::pos2(rect.right(), rect.center().y)],
            egui::Stroke::new(2.0, egui::Color32::from_rgb(180, 180, 185))
        );
        
        // Деления шкалы
        self.render_scale_marks(ui, rect);
        
        // Ползунки
        self.render_slider_knob(ui, current_x, rect.center().y, "Current", 
                               egui::Color32::from_rgb(16, 137, 62)); // Windows зеленый
        self.render_slider_knob(ui, dest_x, rect.center().y, "Destination",
                               egui::Color32::from_rgb(0, 120, 215)); // Windows синий
    }
    
    fn render_scale_marks(&mut self, ui: &mut egui::Ui, rect: egui::Rect) {
        for i in 0..=10 {
            let x = rect.left() + (i as f32 / 10.0) * rect.width();
            let height = if i % 5 == 0 { 12.0 } else { 6.0 };
            let stroke_width = if i % 5 == 0 { 1.5 } else { 1.0 };
            
            ui.painter().line_segment(
                [egui::pos2(x, rect.center().y - height/2.0),
                 egui::pos2(x, rect.center().y + height/2.0)],
                egui::Stroke::new(stroke_width, egui::Color32::from_rgb(120, 120, 125))
            );
            
            if i == 0 || i == 10 {
                let value = if i == 0 { 0.0 } else { 2000.0 };
                ui.painter().text(
                    egui::pos2(x, rect.bottom() + 5.0),
                    egui::Align2::CENTER_TOP,
                    format!("{:.0}", value),
                    egui::FontId::proportional(11.0),
                    egui::Color32::from_rgb(60, 60, 65) // Темно-серый для текста
                );
            }
        }
    }
    
    fn render_slider_knob(&mut self, ui: &mut egui::Ui, x: f32, y: f32, label: &str, color: egui::Color32) {
        let knob_rect = egui::Rect::from_center_size(
            egui::pos2(x, y),
            egui::vec2(24.0, 24.0)
        );
        
        // Основной цвет кружка
        ui.painter().circle_filled(
            knob_rect.center(),
            12.0,
            color
        );
        
        // Граница кружка
        ui.painter().circle_stroke(
            knob_rect.center(),
            12.0,
            egui::Stroke::new(2.0, egui::Color32::from_rgb(240, 240, 245)) // Светлая граница
        );
        
        // Внутренний ободок для объема
        ui.painter().circle_stroke(
            knob_rect.center(),
            10.5,
            egui::Stroke::new(1.0, egui::Color32::WHITE)
        );
        
        // Точка в центре
        ui.painter().circle_filled(
            knob_rect.center(),
            3.0,
            egui::Color32::WHITE
        );
        
        // Цвета меток
        let (label_color, shadow_color) = match label {
            "Current" => (
                egui::Color32::from_rgb(16, 137, 62), // Windows зеленый
                egui::Color32::from_rgb(220, 240, 225) // Светло-зеленый фон
            ),
            "Destination" => (
                egui::Color32::from_rgb(0, 120, 215), // Windows синий
                egui::Color32::from_rgb(220, 235, 250) // Светло-голубой фон
            ),
            _ => (
                egui::Color32::from_rgb(60, 60, 65),
                egui::Color32::from_rgb(240, 240, 245)
            ),
        };
        
        // Фон метки
        let label_bg_rect = egui::Rect::from_center_size(
            egui::pos2(x, knob_rect.top() - 18.0),
            egui::vec2(label.len() as f32 * 6.0 + 12.0, 18.0)
        );
        
        ui.painter().rect_filled(
            label_bg_rect,
            3.0,
            shadow_color
        );
        
        // Текст метки
        ui.painter().text(
            egui::pos2(x, knob_rect.top() - 18.0),
            egui::Align2::CENTER_BOTTOM,
            label,
            egui::FontId::proportional(11.0),
            label_color
        );
    }
}