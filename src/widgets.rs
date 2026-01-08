// src/widgets.rs
use eframe::egui;
use crate::model::PositionerModel;

pub struct CustomWidgets;

impl CustomWidgets {
    pub fn draw_left_panel(ui: &mut egui::Ui, model: &mut PositionerModel) {
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
            
            ui.label(egui::RichText::new("Positioner").size(label_font_size));
            
            let pos_response = ui.allocate_response(
                egui::vec2(positioner_width, positioner_height),
                egui::Sense::hover()
            );
            
            ui.painter().rect_filled(
                pos_response.rect,
                0.0, 
                egui::Color32::from_rgb(40, 40, 45)
            );
            
            ui.painter().rect_stroke(
                pos_response.rect,
                0.0,
                egui::Stroke::new(0.0, egui::Color32::GRAY)
            );
            
            ui.painter().text(
                pos_response.rect.center(),
                egui::Align2::CENTER_CENTER,
                &model.positioner_name,
                egui::FontId::proportional(positioner_value_font_size),
                egui::Color32::WHITE
            );
            
            ui.label(egui::RichText::new("Axis name").size(label_font_size));
            
            let axis_response = ui.allocate_response(
                egui::vec2(axis_width, axis_height),
                egui::Sense::hover()
            );
            
            ui.painter().rect_filled(
                axis_response.rect,
                0.0,
                egui::Color32::from_rgb(40, 40, 45)
            );
            
            ui.painter().rect_stroke(
                axis_response.rect,
                0.0,
                egui::Stroke::new(0.0, egui::Color32::GRAY)
            );
            
            ui.painter().text(
                axis_response.rect.center(),
                egui::Align2::CENTER_CENTER,
                &model.axis_name,
                egui::FontId::proportional(axis_value_font_size),
                egui::Color32::WHITE
            );
            
            ui.add_space(15.0);
            
            let button = egui::Button::new(
                egui::RichText::new("Disconnect")
                    .size(button_font_size)
                    .color(egui::Color32::WHITE)
            ).min_size(egui::vec2(button_width, button_height))
            .fill(egui::Color32::from_rgb(60, 60, 65));
            
            if ui.add(button).clicked() {
                model.disconnect();
            }
        });
    }
    
    pub fn draw_right_panel(ui: &mut egui::Ui, model: &mut PositionerModel) {
        let content_rect = ui.available_rect_before_wrap().shrink(15.0);
        
        ui.allocate_ui_at_rect(content_rect, |ui| {
            ui.vertical_centered(|ui| {
                ui.add_space(10.0);
                
                ui.horizontal(|ui| {
                    ui.label(egui::RichText::new("Repeat").size(12.0));
                    ui.add_space(5.0);
                    ui.add(
                        egui::TextEdit::singleline(&mut model.repeat_value)
                            .desired_width(50.0)
                            .font(egui::TextStyle::Monospace)
                            .hint_text("1")
                    );
                });
                
                ui.add_space(10.0);
            });
        });
    }
    
    pub fn draw_control_section1(ui: &mut egui::Ui, model: &mut PositionerModel) {
        let button_width = 80.0;
        let button_height = 25.0;
        let button_font_size = 12.0;
        let label_font_size = 12.0;
        let element_width = 80.0;
        
        ui.columns(2, |columns| {
            columns[0].vertical_centered(|ui| {
                ui.label(egui::RichText::new("Current, mm").size(label_font_size));
                ui.add_space(5.0);
                
                let current_edit = egui::TextEdit::singleline(&mut model.current_value)
                    .desired_width(element_width)
                    .interactive(true);
                
                let current_response = ui.add(current_edit);
                
                if current_response.changed() || current_response.lost_focus() {
                    model.update_current_from_string();
                }
                
                ui.add_space(5.0);
                
                let button = egui::Button::new(
                    egui::RichText::new("START")
                        .size(button_font_size)
                        .color(egui::Color32::WHITE)
                ).min_size(egui::vec2(button_width, button_height))
                .fill(egui::Color32::from_rgb(60, 60, 65));

                if ui.add(button).clicked() {
                    model.start_movement();
                }
            });
            
            columns[1].vertical_centered(|ui| {
                ui.label(egui::RichText::new("Destination, mm").size(label_font_size));
                ui.add_space(5.0);
                
                let dest_edit = egui::TextEdit::singleline(&mut model.destination_value)
                    .desired_width(element_width)
                    .interactive(true);
                
                let dest_response = ui.add(dest_edit);
                
                if dest_response.changed() || dest_response.lost_focus() {
                    model.update_destination_from_string();
                }
                
                ui.add_space(5.0);
                
                let button = egui::Button::new(
                    egui::RichText::new("STOP")
                        .size(button_font_size)
                        .color(egui::Color32::WHITE)
                ).min_size(egui::vec2(button_width, button_height))
                .fill(egui::Color32::from_rgb(60, 60, 65));

                if ui.add(button).clicked() {
                    model.stop_movement();
                }
            });
        });
    }
    
    pub fn draw_control_section2(ui: &mut egui::Ui, model: &mut PositionerModel) {
        ui.columns(2, |columns| {
            Self::draw_step_control(&mut columns[0], model);
            Self::draw_speed_control(&mut columns[1], model);
        });
    }
    
    fn draw_step_control(ui: &mut egui::Ui, model: &mut PositionerModel) {
        let label_font_size = 12.0;
        
        ui.vertical_centered(|ui| {
            ui.horizontal(|ui| {
                ui.label(egui::RichText::new("Step").size(label_font_size));
                ui.add_space(5.0);
                ui.add(
                    egui::TextEdit::singleline(&mut model.step_value)
                        .desired_width(60.0)
                        .font(egui::TextStyle::Monospace)
                        .hint_text("1/8")
                );
            });
            
            ui.add_space(5.0);
            
            let step_options = ["1/8", "1/4", "1/2", "1"];
            
            let slider_response = ui.add(
                egui::Slider::new(&mut model.step_index, 0..=3)
                    .show_value(false)
                    .text("")
            );
            
            if slider_response.changed() {
                model.step_value = step_options[model.step_index].to_string();
            }
            
            if ui.ctx().input(|i| i.key_pressed(egui::Key::Enter)) {
                if let Some(index) = step_options.iter().position(|&opt| opt == model.step_value) {
                    model.step_index = index;
                }
            }
            
            ui.add_space(3.0);
            ui.horizontal(|ui| {
                let slider_width = slider_response.rect.width();
                let slider_left = slider_response.rect.left();
                
                for (i, &option) in step_options.iter().enumerate() {
                    let step_count = step_options.len();
                    let x_pos = slider_left + (i as f32 / (step_count - 1).max(1) as f32) * slider_width;
                    
                    ui.put(
                        egui::Rect::from_min_size(
                            egui::pos2(x_pos - 15.0, ui.cursor().top()),
                            egui::vec2(30.0, 15.0)
                        ),
                        egui::Label::new(
                            egui::RichText::new(option)
                                .size(10.0)
                                .color(egui::Color32::GRAY)
                        )
                    );
                }
            });
        });
    }
    
    fn draw_speed_control(ui: &mut egui::Ui, model: &mut PositionerModel) {
        let label_font_size = 12.0;
        
        ui.vertical_centered(|ui| {
            ui.horizontal(|ui| {
                ui.label(egui::RichText::new("Speed").size(label_font_size));
                ui.add_space(5.0);
                ui.add(
                    egui::TextEdit::singleline(&mut model.speed_value)
                        .desired_width(40.0)
                        .font(egui::TextStyle::Monospace)
                        .hint_text("0.0")
                );
                ui.label(
                    egui::RichText::new("1/s")
                        .size(11.0)
                        .color(egui::Color32::GRAY)
                );
            });
            
            ui.add_space(5.0);
            
            let min_speed = 0.0;
            let max_speed = 10.0;
            
            if let Ok(speed_num) = model.speed_value.parse::<f32>() {
                let mut speed_temp = speed_num.clamp(min_speed, max_speed);
                let speed_slider = egui::Slider::new(&mut speed_temp, min_speed..=max_speed)
                    .show_value(false)
                    .text("");
                
                let slider_response = ui.add(speed_slider);
                
                if slider_response.changed() {
                    model.speed_value = format!("{:.1}", speed_temp);
                }
                
                ui.add_space(3.0);
                ui.horizontal(|ui| {
                    let slider_width = slider_response.rect.width();
                    let slider_left = slider_response.rect.left();
                    
                    ui.put(
                        egui::Rect::from_min_size(
                            egui::pos2(slider_left, ui.cursor().top()),
                            egui::vec2(15.0, 15.0)
                        ),
                        egui::Label::new(
                            egui::RichText::new("0")
                                .size(10.0)
                                .color(egui::Color32::GRAY)
                        )
                    );
                    
                    ui.put(
                        egui::Rect::from_min_size(
                            egui::pos2(slider_left + slider_width - 15.0, ui.cursor().top()),
                            egui::vec2(15.0, 15.0)
                        ),
                        egui::Label::new(
                            egui::RichText::new("10")
                                .size(10.0)
                                .color(egui::Color32::GRAY)
                        )
                    );
                });
            } else {
                ui.add_space(20.0);
            }
        });
    }
    
    pub fn draw_control_section3(ui: &mut egui::Ui, model: &mut PositionerModel) {
        ui.vertical_centered(|ui| {
            ui.horizontal(|ui| {
                ui.label(egui::RichText::new("Temperature").size(12.0));
                ui.add_space(10.0);
                ui.add(
                    egui::TextEdit::singleline(&mut model.temperature_value)
                        .desired_width(50.0)
                        .interactive(false)
                        .font(egui::TextStyle::Monospace)
                        .frame(false)
                );
            });
            
            ui.add_space(10.0);
            
            ui.horizontal(|ui| {
                ui.label(egui::RichText::new("Voltage").size(12.0));
                ui.add_space(20.0);
                ui.add(
                    egui::TextEdit::singleline(&mut model.voltage_value)
                        .desired_width(50.0)
                        .interactive(false)
                        .font(egui::TextStyle::Monospace)
                        .frame(false)
                );
            });
        });
    }
    
    pub fn draw_slider(ui: &mut egui::Ui, model: &mut PositionerModel) {
        ui.vertical_centered(|ui| {
            let slider_width = model.bottom_width - 20.0;
            let slider_height = 40.0;
            
            ui.add_space(25.0);
            
            let slider_response = ui.allocate_response(
                egui::vec2(slider_width, slider_height),
                egui::Sense::click_and_drag()
            );
            
            let rect = slider_response.rect;
            
            ui.painter().rect_filled(
                rect,
                5.0,
                egui::Color32::from_rgb(60, 60, 70)
            );
            
            let value_to_x = |value: f32| -> f32 {
                let normalized = (value - model.slider_min) / (model.slider_max - model.slider_min);
                rect.left() + normalized * rect.width()
            };
            
            let x_to_value = |x: f32| -> f32 {
                let normalized = (x - rect.left()) / rect.width();
                model.slider_min + normalized * (model.slider_max - model.slider_min)
            };
            
            let current_x = value_to_x(model.slider_current);
            let dest_x = value_to_x(model.slider_destination);
            
            ui.painter().line_segment(
                [egui::pos2(current_x, rect.center().y), 
                 egui::pos2(dest_x, rect.center().y)],
                egui::Stroke::new(3.0, egui::Color32::from_rgb(0, 150, 255))
            );
            
            ui.painter().line_segment(
                [egui::pos2(rect.left(), rect.center().y), 
                 egui::pos2(rect.right(), rect.center().y)],
                egui::Stroke::new(2.0, egui::Color32::from_rgb(100, 100, 110))
            );
            
            for i in 0..=10 {
                let x = rect.left() + (i as f32 / 10.0) * rect.width();
                let height = if i % 5 == 0 { 12.0 } else { 6.0 };
                let stroke_width = if i % 5 == 0 { 1.5 } else { 1.0 };
                
                ui.painter().line_segment(
                    [egui::pos2(x, rect.center().y - height/2.0),
                     egui::pos2(x, rect.center().y + height/2.0)],
                    egui::Stroke::new(stroke_width, egui::Color32::from_rgb(140, 140, 150))
                );
                
                if i == 0 || i == 10 {
                    let value = if i == 0 { model.slider_min } else { model.slider_max };
                    ui.painter().text(
                        egui::pos2(x, rect.bottom() + 5.0),
                        egui::Align2::CENTER_TOP,
                        format!("{:.0}", value),
                        egui::FontId::proportional(11.0),
                        egui::Color32::from_rgb(180, 180, 190)
                    );
                }
            }
            
            let current_rect = egui::Rect::from_center_size(
                egui::pos2(current_x, rect.center().y),
                egui::vec2(24.0, 24.0)
            );
            
            ui.painter().circle_filled(
                current_rect.center(),
                12.0,
                egui::Color32::from_rgb(0, 200, 100)
            );
            
            ui.painter().circle_stroke(
                current_rect.center(),
                12.0,
                egui::Stroke::new(2.0, egui::Color32::WHITE)
            );
            
            ui.painter().text(
                egui::pos2(current_x, current_rect.top() - 18.0),
                egui::Align2::CENTER_BOTTOM,
                "Current",
                egui::FontId::proportional(11.0),
                egui::Color32::from_rgb(0, 200, 150)
            );
            
            let dest_rect = egui::Rect::from_center_size(
                egui::pos2(dest_x, rect.center().y),
                egui::vec2(24.0, 24.0)
            );
            
            ui.painter().circle_filled(
                dest_rect.center(),
                12.0,
                egui::Color32::from_rgb(0, 150, 255)
            );
            
            ui.painter().circle_stroke(
                dest_rect.center(),
                12.0,
                egui::Stroke::new(2.0, egui::Color32::WHITE)
            );
            
            ui.painter().text(
                egui::pos2(dest_x, dest_rect.top() - 18.0),
                egui::Align2::CENTER_BOTTOM,
                "Destination",
                egui::FontId::proportional(11.0),
                egui::Color32::from_rgb(0, 150, 255)
            );
            
            if slider_response.dragged() {
                let pointer_pos = ui.input(|i| i.pointer.interact_pos());
                if let Some(pos) = pointer_pos {
                    let new_value = x_to_value(pos.x);
                    
                    let dist_to_current = (pos.x - current_x).abs();
                    let dist_to_dest = (pos.x - dest_x).abs();
                    
                    if dist_to_current < dist_to_dest {
                        model.slider_current = new_value.clamp(model.slider_min, model.slider_max);
                        model.current_value = format!("{:.1}", model.slider_current);
                    } else {
                        model.slider_destination = new_value.clamp(model.slider_min, model.slider_max);
                        model.destination_value = format!("{:.1}", model.slider_destination);
                    }
                }
            }
            
            ui.add_space(25.0);
        });
    }
    
}