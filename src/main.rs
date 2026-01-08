use eframe::egui;

fn main() -> Result<(), eframe::Error> {
    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default()
            .with_inner_size([1400.0, 800.0])
            .with_title("Управление позиционером"),
        ..Default::default()
    };

    eframe::run_native(
        "Управление позиционером",
        options,
        Box::new(|_cc| Box::<FixedPanelsApp>::default()),
    )
}

struct FixedPanelsApp {
    // ВСЕ РАЗМЕРЫ УКАЗЫВАЮТСЯ ЗДЕСЬ
    panel_width: f32,       // Ширина боковых панелей
    panel_height: f32,      // Высота боковых панелей
    top_height: f32,        // Высота верхней центральной области
    bottom_height: f32,     // Высота нижней центральной области
    
    // Размеры трех секций верхней центральной области
    section1_width: f32,
    section2_width: f32,
    section3_width: f32,
    
    // Ширина нижней центральной панели (теперь задается отдельно!)
    bottom_width: f32,
    
    // ДАННЫЕ ДЛЯ ЛЕВОЙ ПАНЕЛИ
    positioner_name: String,  // Название позиционера
    axis_name: String,        // Название оси

    current_value: String,
    destination_value: String,
    is_moving: bool,

    step_value: String,
    step_index: usize,
    speed_value: String,
    step_value_f32: f32,
}

impl FixedPanelsApp {
    fn new() -> Self {
        Self {
            // Все размеры задаются здесь в одном месте
            panel_width: 120.0,      // Ширина боковых панелей
            panel_height: 200.0,     // Высота боковых панелей
            top_height: 80.0,       // Высота верхней центральной области
            bottom_height: 120.0,    // Высота нижней центральной области
            
            // Ширины трех секций верхней центральной области
            section1_width: 195.0,
            section2_width: 260.0,
            section3_width: 195.0,
            
            // Ширина нижней центральной панели (теперь независимая!)
            bottom_width: 795.0,     // Можно задать любую ширину
            
            // Данные для левой панели
            positioner_name: String::from("Positioner_001"),  // Начальное значение
            axis_name: String::from("TrS"),                   // Название оси

            current_value: String::from("0.0"),
            destination_value: String::from("0.0"),
            is_moving: false,

            step_value: "1/4".to_string(),
            step_index: 1,
            speed_value: "1.0".to_string(),
            step_value_f32: 0.125,
        }
    }
}

impl Default for FixedPanelsApp {
    fn default() -> Self {
        Self::new()
    }
}

impl eframe::App for FixedPanelsApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            egui::ScrollArea::horizontal().show(ui, |ui| {
                // Главный горизонтальный контейнер для ВСЕХ панелей
                ui.horizontal(|ui| {
                    // ЛЕВАЯ ПАНЕЛЬ
                    let left_response = ui.allocate_response(
                        egui::vec2(self.panel_width, self.panel_height),
                        egui::Sense::hover()
                    );
                    
                    // Рисуем рамку левой панели
                    ui.painter().rect_stroke(
                        left_response.rect,
                        2.0,
                        egui::Stroke::new(0.1, egui::Color32::WHITE)
                    );
                    
                    // Содержимое левой панели
                    ui.allocate_ui_at_rect(left_response.rect, |ui| {
                        ui.vertical_centered(|ui| {
                            let label_font_size = 12.0;
                            let positioner_value_font_size = 14.0;
                            let axis_value_font_size = 28.0;
                            
                            let positioner_width = 118.0;   // Ширина positioner Frame
                            let positioner_height = 40.0;   // Высота positioner Frame
                            let axis_width = 118.0;         // Ширина axis Frame  
                            let axis_height = 50.0;         // Высота axis Frame 
                            
                            let button_width = 80.0;
                            let button_height = 25.0;
                            let button_font_size = 12.0;
                            
                            ui.label(
                                egui::RichText::new("Positioner")
                                    .size(label_font_size)
                            );
                            
                            let pos_response = ui.allocate_response(
                                egui::vec2(positioner_width, positioner_height),
                                egui::Sense::hover()
                            );
                            
                            // Рисуем Frame
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
                                &self.positioner_name,
                                egui::FontId::proportional(positioner_value_font_size),
                                egui::Color32::WHITE
                            );
                            
                            ui.label(
                                egui::RichText::new("Axis name")
                                    .size(label_font_size)
                            );
                            
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
                                &self.axis_name,
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
                                println!("Попытка отключения от позиционера...");
                            }
                        });
                    });
                    
                    ui.add_space(1.0);
                    
                    // ЦЕНТРАЛЬНАЯ ОБЛАСТЬ
                    ui.vertical(|ui| {
                        // ВЕРХНЯЯ ЧАСТЬ - 3 СЕКЦИИ РЯДОМ
                        ui.horizontal(|ui| {
                            // Секция 1
                            let sec1_response = ui.allocate_response(
                                egui::vec2(self.section1_width, self.top_height),
                                egui::Sense::hover()
                            );
                            
                            ui.painter().rect_stroke(
                                sec1_response.rect,
                                1.0,
                                egui::Stroke::new(0.1, egui::Color32::WHITE)
                            );
                            let button_width = 80.0;
                            let button_height = 25.0;
                            let button_font_size = 12.0;
                            
                            ui.allocate_ui_at_rect(sec1_response.rect, |ui| {
                                ui.columns(2, |columns| {
                                    // ОБЩИЕ ПАРАМЕТРЫ
                                    let label_font_size = 12.0;
                                    let element_width = 80.0;
                                    
                                    // ЛЕВАЯ КОЛОНКА - Current
                                    columns[0].vertical_centered(|ui| {
                                        // Заголовок Current
                                        ui.label(
                                            egui::RichText::new("Current, mm")
                                                .size(label_font_size)
                                        );
                                        
                                        ui.add_space(5.0);
                                        
                                        // Current с текущим значением
                                        ui.add(
                                            egui::TextEdit::singleline(&mut self.current_value)
                                                .desired_width(element_width)
                                                .interactive(false) // Нельзя редактировать
                                                
                                        );
                                        
                                        ui.add_space(5.0);
                                        
                                        // Кнопка START
                                        let button = egui::Button::new(
                                            egui::RichText::new("START")
                                                .size(button_font_size)
                                                .color(egui::Color32::WHITE)
                                        ).min_size(egui::vec2(button_width, button_height))
                                        .fill(egui::Color32::from_rgb(60, 60, 65));

                                        if ui.add(button).clicked() {
                                            self.is_moving = true;
                                            println!("Начато движение к: {}", self.destination_value);
                                        }
                                        
                                    });
                                    
                                    // ПРАВАЯ КОЛОНКА - Destination
                                    columns[1].vertical_centered(|ui| {
                                        // Заголовок Destination
                                        ui.label(
                                            egui::RichText::new("Destination, mm")
                                                .size(label_font_size)
                                        );
                                        
                                        ui.add_space(5.0);
                                        
                                        // LineEdit БЕЗ Frame, с таким же размером
                                        ui.add(
                                            egui::TextEdit::singleline(&mut self.destination_value)
                                                .desired_width(element_width)
                                        );
                                        
                                        ui.add_space(5.0);
                                        
                                        // Кнопка STOP
                                        let button = egui::Button::new(
                                            egui::RichText::new("STOP")
                                                .size(button_font_size)
                                                .color(egui::Color32::WHITE)
                                        ).min_size(egui::vec2(button_width, button_height))
                                        .fill(egui::Color32::from_rgb(60, 60, 65));

                                        if ui.add(button).clicked() {
                                            self.is_moving = false;
                                            println!("ОСТАНОВКА ДВИЖЕНИЯ");
                                        }
                                    });
                                });
                            });
                            
                            let sec2_response = ui.allocate_response(
                                egui::vec2(self.section2_width, self.top_height),
                                egui::Sense::hover()
                            );
                            
                            ui.painter().rect_stroke(
                                sec2_response.rect,
                                1.0,
                                egui::Stroke::new(0.1, egui::Color32::WHITE)
                            );
                            
                            ui.allocate_ui_at_rect(sec2_response.rect, |ui| {
                                // Переменные для регулировки отступов
                                let left_padding = 15.0;  // Отступ от левого края (можно менять)
                                let top_padding = 8.0;    // Отступ от верхнего края (можно менять)
                                
                                // Создаем внутренний контейнер с отступами
                                ui.allocate_ui_at_rect(
                                    egui::Rect::from_min_max(
                                        sec2_response.rect.min + egui::vec2(left_padding, top_padding),
                                        sec2_response.rect.max
                                    ),
                                    |ui| {
                                        ui.columns(2, |columns| {
                                            // ОБЩИЕ ПАРАМЕТРЫ
                                            let label_font_size = 12.0;
                                            
                                            // ЛЕВАЯ КОЛОНКА - Step
                                            columns[0].vertical_centered(|ui| {
                                                // Step и LineEdit в одной строке
                                                ui.horizontal(|ui| {
                                                    // Метка Step
                                                    ui.label(
                                                        egui::RichText::new("Step")
                                                            .size(label_font_size)
                                                    );
                                                    
                                                    ui.add_space(5.0);
                                                    
                                                    // LineEdit для ввода дроби
                                                    ui.add(
                                                        egui::TextEdit::singleline(&mut self.step_value)
                                                            .desired_width(60.0)
                                                            .font(egui::TextStyle::Monospace)
                                                            .hint_text("1/8")
                                                    );
                                                });
                                                
                                                ui.add_space(5.0);
                                                
                                                // Ползунок для Step с отметками
                                                let step_options = ["1/8", "1/4", "1/2", "1"];
                                                
                                                // Ползунок
                                                let slider_response = ui.add(
                                                    egui::Slider::new(&mut self.step_index, 0..=3)
                                                        .show_value(false)
                                                        .text("")
                                                );
                                                
                                                // Проверяем изменение ползунка
                                                if slider_response.changed() {
                                                    // Обновляем значение в LineEdit при движении ползунка
                                                    self.step_value = step_options[self.step_index].to_string();
                                                }
                                                
                                                // Обработка ввода в LineEdit
                                                if ui.ctx().input(|i| i.key_pressed(egui::Key::Enter)) {
                                                    // При нажатии Enter обновляем индекс на основе введенного значения
                                                    if let Some(index) = step_options.iter().position(|&opt| opt == self.step_value) {
                                                        self.step_index = index;
                                                    }
                                                }
                                                
                                                // Подписи значений под ползунком Step
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
                                            
                                            // ПРАВАЯ КОЛОНКА - Speed
                                            columns[1].vertical_centered(|ui| {
                                                // Speed и LineEdit в одной строке
                                                ui.horizontal(|ui| {
                                                    // Метка Speed
                                                    ui.label(
                                                        egui::RichText::new("Speed")
                                                            .size(label_font_size)
                                                    );
                                                    
                                                    ui.add_space(5.0);
                                                    
                                                    // Поле ввода для Speed (LineEdit)
                                                    ui.add(
                                                        egui::TextEdit::singleline(&mut self.speed_value)
                                                            .desired_width(40.0)
                                                            .font(egui::TextStyle::Monospace)
                                                            .hint_text("0.0")
                                                    );
                                                    
                                                    // Метка "1/s"
                                                    ui.label(
                                                        egui::RichText::new("1/s")
                                                            .size(11.0)
                                                            .color(egui::Color32::GRAY)
                                                    );
                                                });
                                                
                                                ui.add_space(5.0);
                                                
                                                // Ползунок для Speed
                                                let min_speed = 0.0;
                                                let max_speed = 10.0;
                                                
                                                if let Ok(speed_num) = self.speed_value.parse::<f32>() {
                                                    let mut speed_temp = speed_num.clamp(min_speed, max_speed);
                                                    let speed_slider = egui::Slider::new(&mut speed_temp, min_speed..=max_speed)
                                                        .show_value(false)
                                                        .text("");
                                                    
                                                    let slider_response = ui.add(speed_slider);
                                                    
                                                    if slider_response.changed() {
                                                        self.speed_value = format!("{:.1}", speed_temp);
                                                    }
                                                    
                                                    // Подписи минимального и максимального значения под ползунком Speed
                                                    ui.add_space(3.0);
                                                    ui.horizontal(|ui| {
                                                        let slider_width = slider_response.rect.width();
                                                        let slider_left = slider_response.rect.left();
                                                        
                                                        // Минимальное значение (слева)
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
                                                        
                                                        // Максимальное значение (справа)
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
                                                    // Если значение некорректное, показываем placeholder
                                                    ui.add_space(20.0); // Отступ вместо ползунка
                                                }
                                            });
                                        });
                                    }
                                );
                            });
                            
                            // Секция 3
                            let sec3_response = ui.allocate_response(
                                egui::vec2(self.section3_width, self.top_height),
                                egui::Sense::hover()
                            );
                            
                            ui.painter().rect_stroke(
                                sec3_response.rect,
                                5.0,
                                egui::Stroke::new(2.0, egui::Color32::DARK_GREEN)
                            );
                            
                            ui.allocate_ui_at_rect(sec3_response.rect, |ui| {
                                ui.vertical_centered(|ui| {
                                    ui.heading("Секция 3");
                                    ui.separator();
                                    ui.label("5.2.6 Ползунок");
                                    ui.label("5.2.7 Сброс");
                                    ui.label("Калибровка");
                                });
                            });
                        });
                        
                        // Небольшой отступ между верхней и нижней частями
                        ui.add_space(5.0);
                        
                        // НИЖНЯЯ ЧАСТЬ - ОДНА ПАНЕЛЬ (независимая ширина!)
                        let bottom_response = ui.allocate_response(
                            egui::vec2(self.bottom_width, self.bottom_height),
                            egui::Sense::hover()
                        );
                        
                        ui.painter().rect_stroke(
                            bottom_response.rect,
                            5.0,
                            egui::Stroke::new(2.0, egui::Color32::RED)
                        );
                        
                        ui.allocate_ui_at_rect(bottom_response.rect, |ui| {
                            ui.vertical_centered(|ui| {
                                ui.heading("Нижняя область");
                                ui.separator();
                                ui.label(format!("{} × {} px", self.bottom_width, self.bottom_height));
                                ui.horizontal(|ui| {
                                    ui.label("Текущая: 1250");
                                    ui.add_space(20.0);
                                    ui.label("Целевая: 1500");
                                });
                                ui.horizontal(|ui| {
                                    if ui.button("Старт").clicked() {}
                                    if ui.button("Стоп").clicked() {}
                                    if ui.button("Сброс").clicked() {}
                                });
                            });
                        });
                    });
                    
                    // Небольшой отступ между панелями
                    ui.add_space(5.0);
                    
                    // ПРАВАЯ ПАНЕЛЬ
                    let right_response = ui.allocate_response(
                        egui::vec2(self.panel_width, self.panel_height),
                        egui::Sense::hover()
                    );
                    
                    // Рисуем рамку правой панели
                    ui.painter().rect_stroke(
                        right_response.rect,
                        5.0,
                        egui::Stroke::new(2.0, egui::Color32::from_rgb(128, 0, 128))
                    );
                    
                    // Содержимое правой панели
                    ui.allocate_ui_at_rect(right_response.rect, |ui| {
                        ui.vertical_centered(|ui| {
                            ui.heading("Правая панель");
                            ui.separator();
                            ui.label(format!("{} × {} px", self.panel_width, self.panel_height));
                            ui.separator();
                            if ui.button("Подключить").clicked() {}
                            if ui.button("Отключить").clicked() {}
                            if ui.button("Калибровка").clicked() {}
                            ui.separator();
                            ui.label("Статус:");
                            ui.label("✓ USB подключен");
                            ui.label("⚠ Двигатель выкл.");
                        });
                    });
                });
            });
        });
    }
}