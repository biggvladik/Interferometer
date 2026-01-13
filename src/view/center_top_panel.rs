use eframe::egui;
use crate::controller::AppController;
use super::PanelComponent;

pub struct CenterTopPanel;

impl PanelComponent for CenterTopPanel {
    fn render(&mut self, ui: &mut egui::Ui, controller: &mut AppController) {
        ui.horizontal(|ui| {
            // Секция 1
            self.render_section1(ui, controller);
            
            // Секция 2
            self.render_section2(ui, controller);
            
            // Секция 3
            self.render_section3(ui, controller);
        });
    }
}

impl CenterTopPanel {
    fn render_section1(&mut self, ui: &mut egui::Ui, controller: &mut AppController) {
        let sec1_frame = egui::Frame::none()
            .inner_margin(4.0)
            .stroke(egui::Stroke::new(0.1, egui::Color32::WHITE));
        
        sec1_frame.show(ui, |ui| {
            ui.set_width(controller.config.section1_width);
            ui.set_height(controller.config.top_height);
            
            let button_width = 80.0;
            let button_height = 25.0;
            let button_font_size = 12.0;
            
            ui.columns(2, |columns| {
                let label_font_size = 12.0;
                let element_width = 80.0;
                
                // ЛЕВАЯ КОЛОНКА - Current
                columns[0].vertical_centered(|ui| {
                    ui.label(
                        egui::RichText::new("Current, mm")
                            .size(label_font_size)
                    );
                    
                    ui.add_space(5.0);
                    
                    let current_edit = egui::TextEdit::singleline(&mut controller.model.current_value)
                        .desired_width(element_width)
                        .interactive(true);
                    
                    let current_response = ui.add(current_edit);
                    
                    if current_response.changed() || current_response.lost_focus() {
                        controller.update_current_from_string();
                    }
                    
                    ui.add_space(5.0);
                    
                    let button = egui::Button::new(
                        egui::RichText::new("START")
                            .size(button_font_size)
                            .color(egui::Color32::WHITE)
                    ).min_size(egui::vec2(button_width, button_height))
                    .fill(egui::Color32::from_rgb(60, 60, 65));

                    if ui.add(button).clicked() {
                        controller.handle_start_movement();
                    }
                });
                
                // ПРАВАЯ КОЛОНКА - Destination
                columns[1].vertical_centered(|ui| {
                    ui.label(
                        egui::RichText::new("Destination, mm")
                            .size(label_font_size)
                    );
                    
                    ui.add_space(5.0);
                    
                    let dest_edit = egui::TextEdit::singleline(&mut controller.model.destination_value)
                        .desired_width(element_width)
                        .interactive(true);
                    
                    let dest_response = ui.add(dest_edit);
                    
                    if dest_response.changed() || dest_response.lost_focus() {
                        controller.update_destination_from_string();
                    }
                    
                    ui.add_space(5.0);
                    
                    let button = egui::Button::new(
                        egui::RichText::new("STOP")
                            .size(button_font_size)
                            .color(egui::Color32::WHITE)
                    ).min_size(egui::vec2(button_width, button_height))
                    .fill(egui::Color32::from_rgb(60, 60, 65));

                    if ui.add(button).clicked() {
                        controller.handle_stop_movement();
                    }
                });
            });
        });
    }
    
    fn render_section2(&mut self, ui: &mut egui::Ui, controller: &mut AppController) {
        let sec2_frame = egui::Frame::none()
            .inner_margin(egui::Margin {
                left: 15.0,
                right: 0.0,
                top: 8.0,
                bottom: 0.0,
            })
            .stroke(egui::Stroke::new(0.1, egui::Color32::WHITE));
        
        sec2_frame.show(ui, |ui| {
            ui.set_width(controller.config.section2_width);
            ui.set_height(controller.config.top_height);
            
            ui.columns(2, |columns| {
                let label_font_size = 12.0;
                
                // ЛЕВАЯ КОЛОНКА - Step
                columns[0].vertical_centered(|ui| {
                    ui.horizontal(|ui| {
                        ui.label(
                            egui::RichText::new("Step")
                                .size(label_font_size)
                        );
                        
                        ui.add_space(5.0);
                        
                        ui.add(
                            egui::TextEdit::singleline(&mut controller.model.step_value)
                                .desired_width(60.0)
                                .font(egui::TextStyle::Monospace)
                                .hint_text("1/8")
                        );
                    });
                    
                    ui.add_space(5.0);
                    
                    let step_options = ["1/8", "1/4", "1/2", "1"];
                    
                    let slider_response = ui.add(
                        egui::Slider::new(&mut controller.model.step_index, 0..=3)
                            .show_value(false)
                            .text("")
                    );
                    
                    if slider_response.changed() {
                        controller.model.step_value = step_options[controller.model.step_index].to_string();
                    }
                    
                    if ui.ctx().input(|i| i.key_pressed(egui::Key::Enter)) {
                        if let Some(index) = step_options.iter().position(|&opt| opt == controller.model.step_value) {
                            controller.model.step_index = index;
                        }
                    }
                    
                    ui.add_space(3.0);
                    self.render_step_labels(ui, step_options, slider_response.rect);
                });
                
                // ПРАВАЯ КОЛОНКА - Speed
                columns[1].vertical_centered(|ui| {
                    ui.horizontal(|ui| {
                        ui.label(
                            egui::RichText::new("Speed")
                                .size(label_font_size)
                        );
                        
                        ui.add_space(5.0);
                        
                        ui.add(
                            egui::TextEdit::singleline(&mut controller.model.speed_value)
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
                    
                    self.render_speed_slider(ui, controller);
                });
            });
        });
    }
    
    fn render_step_labels(&mut self, ui: &mut egui::Ui, step_options: [&str; 4], slider_rect: egui::Rect) {
        ui.horizontal(|ui| {
            let slider_width = slider_rect.width();
            let slider_left = slider_rect.left();
            
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
    }
    
    fn render_speed_slider(&mut self, ui: &mut egui::Ui, controller: &mut AppController) {
        let min_speed = 0.0;
        let max_speed = 10.0;
        
        if let Ok(speed_num) = controller.model.speed_value.parse::<f32>() {
            let mut speed_temp = speed_num.clamp(min_speed, max_speed);
            let speed_slider = egui::Slider::new(&mut speed_temp, min_speed..=max_speed)
                .show_value(false)
                .text("");
            
            let slider_response = ui.add(speed_slider);
            
            if slider_response.changed() {
                controller.model.speed_value = format!("{:.1}", speed_temp);
            }
            
            ui.add_space(3.0);
            self.render_speed_labels(ui, slider_response.rect);
        } else {
            ui.add_space(20.0);
        }
    }
    
    fn render_speed_labels(&mut self, ui: &mut egui::Ui, slider_rect: egui::Rect) {
        ui.horizontal(|ui| {
            let slider_width = slider_rect.width();
            let slider_left = slider_rect.left();
            
            // Минимальное значение
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
            
            // Максимальное значение
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
    }
    
    fn render_section3(&mut self, ui: &mut egui::Ui, controller: &mut AppController) {
        let sec3_frame = egui::Frame::none()
            .inner_margin(4.0)
            .stroke(egui::Stroke::new(0.1, egui::Color32::WHITE));
        
        sec3_frame.show(ui, |ui| {
            ui.set_width(controller.config.section3_width);
            ui.set_height(controller.config.top_height);
            
            ui.vertical_centered(|ui| {
                // Температура
                ui.horizontal(|ui| {
                    ui.label(
                        egui::RichText::new("Temperature")
                            .size(12.0)
                    );
                    
                    ui.add_space(10.0);
                    
                    ui.add(
                        egui::TextEdit::singleline(&mut controller.model.temperature_value)
                            .desired_width(50.0)
                            .interactive(false)
                            .font(egui::TextStyle::Monospace)
                            .frame(false)
                    );
                });
                
                ui.add_space(10.0);
                
                // Напряжение
                ui.horizontal(|ui| {
                    ui.label(
                        egui::RichText::new("Voltage")
                            .size(12.0)
                    );
                    
                    ui.add_space(20.0);
                    
                    ui.add(
                        egui::TextEdit::singleline(&mut controller.model.voltage_value)
                            .desired_width(50.0)
                            .interactive(false)
                            .font(egui::TextStyle::Monospace)
                            .frame(false)
                    );
                });
            });
        });
    }
}