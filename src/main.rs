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

    temperature_value: String,
    voltage_value: String,
    
    // ДАННЫЕ ДЛЯ ПРАВОЙ ПАНЕЛИ
    repeat_value: String,     // Новое поле для Repeat
    
    // ДАННЫЕ ДЛЯ СЛАЙДЕРА В НИЖНЕЙ ПАНЕЛИ
    slider_min: f32,          // Минимальное значение слайдера
    slider_max: f32,          // Максимальное значение слайдера
    slider_current: f32,      // Текущая позиция (первый ползунок)
    slider_destination: f32,  // Целевая позиция (второй ползунок)
}

impl FixedPanelsApp {
    fn new() -> Self {
        Self {
            // Все размеры задаются здесь в одном месте
            panel_width: 120.0,      // Ширина боковых панелей
            panel_height: 200.0,     // Высота боковых панелей
            top_height: 80.0,       // Высота верхней центральной области
            bottom_height: 105.0,    // Высота нижней центральной области
            
            // Ширины трех секций верхней центральной области
            section1_width: 195.0,
            section2_width: 260.0,
            section3_width: 150.0,
            
            // Ширина нижней центральной панели (теперь независимая!)
            bottom_width: 684.0,     // Можно задать любую ширину
            
            // Данные для левой панели
            positioner_name: String::from("Positioner_001"),  // Начальное значение
            axis_name: String::from("TrS"),                   // Название оси

            current_value: String::from("1250.0"),     // Начальное значение синхронизировано со слайдером
            destination_value: String::from("1500.0"), // Начальное значение синхронизировано со слайдером
            is_moving: false,

            step_value: "1/4".to_string(),
            step_index: 1,
            speed_value: "1.0".to_string(),

            temperature_value: "25°C".to_string(),
            voltage_value: "24V".to_string(),
            
            // Данные для правой панели
            repeat_value: "1".to_string(),            // Начальное значение Repeat
            
            // Инициализация данных для слайдера
            slider_min: 0.0,
            slider_max: 2000.0,
            slider_current: 1250.0,     // Начальная текущая позиция
            slider_destination: 1500.0,  // Начальная целевая позиция
        }
    }
    
    // Метод для синхронизации значений из строк в числа и обратно
    fn update_current_from_string(&mut self) {
        if let Ok(value) = self.current_value.parse::<f32>() {
            self.slider_current = value.clamp(self.slider_min, self.slider_max);
            // Обновляем строковое значение, чтобы оно соответствовало границам
            self.current_value = format!("{:.1}", self.slider_current);
        }
    }
    
    fn update_destination_from_string(&mut self) {
        if let Ok(value) = self.destination_value.parse::<f32>() {
            self.slider_destination = value.clamp(self.slider_min, self.slider_max);
            // Обновляем строковое значение, чтобы оно соответствовало границам
            self.destination_value = format!("{:.1}", self.slider_destination);
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
        // Синхронизация при нажатии Enter в полях ввода
        if ctx.input(|i| i.key_pressed(egui::Key::Enter)) {
            self.update_current_from_string();
            self.update_destination_from_string();
        }
        
        egui::CentralPanel::default().show(ctx, |ui| {
            egui::ScrollArea::horizontal().show(ui, |ui| {
                // Главный горизонтальный контейнер для ВСЕХ панелей
                ui.horizontal(|ui| {
                    // ЛЕВАЯ ПАНЕЛЬ (остается без изменений)
                    let left_response = ui.allocate_response(
                        egui::vec2(self.panel_width, self.panel_height),
                        egui::Sense::hover()
                    );
                    
                    // Рисуем рамку левой панели
                    ui.painter().rect_stroke(
                        left_response.rect,
                        1.0,
                        egui::Stroke::new(0.1, egui::Color32::WHITE)
                    );
                    
                    // Содержимое левой панели (без изменений)
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
                    
                    // ЦЕНТРАЛЬНАЯ ОБЛАСТЬ (верхняя часть)
                    ui.vertical(|ui| {
                        // ВЕРХНЯЯ ЧАСТЬ - 3 СЕКЦИИ РЯДОМ
                        ui.horizontal(|ui| {
                            // Секция 1 - используем Frame для правильной компоновки
                            let sec1_frame = egui::Frame::none()
                                .inner_margin(4.0)
                                .stroke(egui::Stroke::new(0.1, egui::Color32::WHITE));
                            
                            sec1_frame.show(ui, |ui| {
                                ui.set_width(self.section1_width);
                                ui.set_height(self.top_height);
                                
                                let button_width = 80.0;
                                let button_height = 25.0;
                                let button_font_size = 12.0;
                                
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
                                        
                                        // Current с текущим значением - СИНХРОНИЗИРОВАНО СО СЛАЙДЕРОМ
                                        let current_edit = egui::TextEdit::singleline(&mut self.current_value)
                                            .desired_width(element_width)
                                            .interactive(true); // Можно редактировать
                                        
                                        let current_response = ui.add(current_edit);
                                        
                                        // Если поле изменилось, синхронизируем со слайдером
                                        if current_response.changed() || current_response.lost_focus() {
                                            self.update_current_from_string();
                                        }
                                        
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
                                            println!("Начато движение от {} к {}", 
                                                    self.slider_current, self.slider_destination);
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
                                        
                                        // LineEdit для Destination - СИНХРОНИЗИРОВАНО СО СЛАЙДЕРОМ
                                        let dest_edit = egui::TextEdit::singleline(&mut self.destination_value)
                                            .desired_width(element_width)
                                            .interactive(true);
                                        
                                        let dest_response = ui.add(dest_edit);
                                        
                                        // Если поле изменилось, синхронизируем со слайдером
                                        if dest_response.changed() || dest_response.lost_focus() {
                                            self.update_destination_from_string();
                                        }
                                        
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
                            
                            // Секция 2 - тоже используем Frame с дополнительными отступами
                            let sec2_frame = egui::Frame::none()
                                .inner_margin(egui::Margin {
                                    left: 15.0,    // Отступ слева от границы
                                    right: 0.0,
                                    top: 8.0,      // Отступ сверху от границы
                                    bottom: 0.0,
                                })
                                .stroke(egui::Stroke::new(0.1, egui::Color32::WHITE));
                            
                            sec2_frame.show(ui, |ui| {
                                ui.set_width(self.section2_width);
                                ui.set_height(self.top_height);
                                
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
                            });
                            
                            // Секция 3 - ОДНА КОЛОНКА с температурой и напряжением
                            let sec3_frame = egui::Frame::none()
                                .inner_margin(4.0)
                                .stroke(egui::Stroke::new(0.1, egui::Color32::WHITE));
                            
                            sec3_frame.show(ui, |ui| {
                                ui.set_width(self.section3_width);
                                ui.set_height(self.top_height);
                                
                                // Одна колонка с центрированием
                                ui.vertical_centered(|ui| {
                                    // Температура
                                    ui.horizontal(|ui| {
                                        ui.label(
                                            egui::RichText::new("Temperature")
                                                .size(12.0)
                                        );
                                        
                                        ui.add_space(10.0);
                                        
                                        // Просто значение температуры без прямоугольника
                                        ui.add(
                                            egui::TextEdit::singleline(&mut self.temperature_value)
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
                                        
                                        // Просто значение напряжения без прямоугольника
                                        ui.add(
                                            egui::TextEdit::singleline(&mut self.voltage_value)
                                                .desired_width(50.0)
                                                .interactive(false)
                                                .font(egui::TextStyle::Monospace)
                                                .frame(false)
                                        );
                                    });
                                });
                            });
                        });
                        
                        // Небольшой отступ между верхней и нижней частями
                        ui.add_space(5.0);
                        
                        // НИЖНЯЯ ЧАСТЬ - БОЛЬШОЙ СЛАЙДЕР ПОСЕРЕДИНЕ
                        let bottom_response = ui.allocate_response(
                            egui::vec2(self.bottom_width, self.bottom_height),
                            egui::Sense::hover()
                        );
                        
                        ui.painter().rect_stroke(
                            bottom_response.rect,
                            1.0,
                            egui::Stroke::new(0.1, egui::Color32::WHITE)
                        );
                        
                        ui.allocate_ui_at_rect(bottom_response.rect, |ui| {
                            ui.vertical_centered(|ui| {
                                // БОЛЬШОЙ СЛАЙДЕР, ЗАНИМАЮЩИЙ БОЛЬШУЮ ЧАСТЬ ПРОСТРАНСТВА
                                let slider_width = self.bottom_width - 20.0; // Уменьшаем отступы, чтобы подписи поместились
                                let slider_height = 40.0;
                                
                                ui.add_space(25.0);
                                
                                // Выделяем область для слайдера
                                let slider_response = ui.allocate_response(
                                    egui::vec2(slider_width, slider_height),
                                    egui::Sense::click_and_drag()
                                );
                                
                                let rect = slider_response.rect;
                                
                                // Рисуем фон слайдера
                                ui.painter().rect_filled(
                                    rect,
                                    5.0,
                                    egui::Color32::from_rgb(60, 60, 70)
                                );
                                
                                // Вычисляем позиции ползунков
                                let value_to_x = |value: f32| -> f32 {
                                    let normalized = (value - self.slider_min) / (self.slider_max - self.slider_min);
                                    rect.left() + normalized * rect.width()
                                };
                                
                                let x_to_value = |x: f32| -> f32 {
                                    let normalized = (x - rect.left()) / rect.width();
                                    self.slider_min + normalized * (self.slider_max - self.slider_min)
                                };
                                
                                let current_x = value_to_x(self.slider_current);
                                let dest_x = value_to_x(self.slider_destination);
                                
                                // Рисуем линию между ползунками
                                ui.painter().line_segment(
                                    [egui::pos2(current_x, rect.center().y), 
                                     egui::pos2(dest_x, rect.center().y)],
                                    egui::Stroke::new(3.0, egui::Color32::from_rgb(0, 150, 255))
                                );
                                
                                // Рисуем шкалу (основную линию)
                                ui.painter().line_segment(
                                    [egui::pos2(rect.left(), rect.center().y), 
                                     egui::pos2(rect.right(), rect.center().y)],
                                    egui::Stroke::new(2.0, egui::Color32::from_rgb(100, 100, 110))
                                );
                                
                                // Добавляем деления шкалы и подписи под ними
                                for i in 0..=10 {
                                    let x = rect.left() + (i as f32 / 10.0) * rect.width();
                                    let height = if i % 5 == 0 { 12.0 } else { 6.0 };
                                    let stroke_width = if i % 5 == 0 { 1.5 } else { 1.0 };
                                    
                                    ui.painter().line_segment(
                                        [egui::pos2(x, rect.center().y - height/2.0),
                                         egui::pos2(x, rect.center().y + height/2.0)],
                                        egui::Stroke::new(stroke_width, egui::Color32::from_rgb(140, 140, 150))
                                    );
                                    
                                    // Подписи для основных делений (каждое 5-ое) - теперь только 0 и 2000
                                    if i == 0 || i == 10 {
                                        let value = if i == 0 { self.slider_min } else { self.slider_max };
                                        // Подпись располагаем по центру под делением
                                        ui.painter().text(
                                            egui::pos2(x, rect.bottom() + 5.0),
                                            egui::Align2::CENTER_TOP,
                                            format!("{:.0}", value),
                                            egui::FontId::proportional(11.0),
                                            egui::Color32::from_rgb(180, 180, 190)
                                        );
                                    }
                                }
                                
                                // Первый ползунок (Current - зеленый)
                                let current_rect = egui::Rect::from_center_size(
                                    egui::pos2(current_x, rect.center().y),
                                    egui::vec2(24.0, 24.0)
                                );
                                
                                ui.painter().circle_filled(
                                    current_rect.center(),
                                    12.0,
                                    egui::Color32::from_rgb(0, 200, 100)
                                );
                                
                                // Белая обводка для ползунка
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
                                
                                // Второй ползунок (Destination - синий)
                                let dest_rect = egui::Rect::from_center_size(
                                    egui::pos2(dest_x, rect.center().y),
                                    egui::vec2(24.0, 24.0)
                                );
                                
                                ui.painter().circle_filled(
                                    dest_rect.center(),
                                    12.0,
                                    egui::Color32::from_rgb(0, 150, 255)
                                );
                                
                                // Белая обводка для ползунка
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
                                
                                // Обработка взаимодействия с ползунками
                                if slider_response.dragged() {
                                    let pointer_pos = ui.input(|i| i.pointer.interact_pos());
                                    if let Some(pos) = pointer_pos {
                                        let new_value = x_to_value(pos.x);
                                        
                                        // Определяем, какой ползунок ближе
                                        let dist_to_current = (pos.x - current_x).abs();
                                        let dist_to_dest = (pos.x - dest_x).abs();
                                        
                                        if dist_to_current < dist_to_dest {
                                            self.slider_current = new_value.clamp(self.slider_min, self.slider_max);
                                            // Синхронизируем со строковым значением
                                            self.current_value = format!("{:.1}", self.slider_current);
                                        } else {
                                            self.slider_destination = new_value.clamp(self.slider_min, self.slider_max);
                                            // Синхронизируем со строковым значением
                                            self.destination_value = format!("{:.1}", self.slider_destination);
                                        }
                                    }
                                }
                                
                                ui.add_space(25.0);
                            });
                        });
                    });
                    
                    // Небольшой отступ между панелями
                    ui.add_space(5.0);
                    
                    // ПРАВАЯ ПАНЕЛЬ (с добавленным Repeat в одну строку и отступом от правой границы)
                    let right_response = ui.allocate_response(
                        egui::vec2(self.panel_width, self.panel_height),
                        egui::Sense::hover()
                    );
                    
                    // Рисуем рамку правой панели
                    ui.painter().rect_stroke(
                        right_response.rect,
                        1.0,
                        egui::Stroke::new(0.1, egui::Color32::WHITE)
                    );
                    
                    // Содержимое правой панели С ОТСТУПОМ ОТ ГРАНИЦ
                    ui.allocate_ui_at_rect(right_response.rect, |ui| {
                        // Создаем меньшую область с отступами
                        let content_rect = right_response.rect.shrink(15.0); // Отступ по 15px со всех сторон
                        
                        // Используем эту область для размещения содержимого
                        ui.allocate_ui_at_rect(content_rect, |ui| {
                            ui.vertical_centered(|ui| {
                                // Отступ сверху
                                ui.add_space(10.0);
                                
                                // Label и LineEdit для Repeat В ОДНУ СТРОКУ
                                // Аналогично тому, как сделано для Step и Speed
                                ui.horizontal(|ui| {
                                    // Метка Repeat - слева
                                    ui.label(
                                        egui::RichText::new("Repeat")
                                            .size(12.0)  // Такой же размер шрифта, как у Step и Speed
                                    );
                                    
                                    // Отступ между Label и LineEdit
                                    ui.add_space(5.0);
                                    
                                    // LineEdit для Repeat - справа
                                    // Уменьшаем ширину, чтобы поместиться в отступах
                                    ui.add(
                                        egui::TextEdit::singleline(&mut self.repeat_value)
                                            .desired_width(50.0)  // Немного уменьшили ширину
                                            .font(egui::TextStyle::Monospace)  // Моноширинный шрифт
                                            .hint_text("1")  // Подсказка
                                    );
                                });
                                
                                // Отступ снизу для баланса
                                ui.add_space(10.0);
                            });
                        });
                    });
                });
            });
        });
    }
}