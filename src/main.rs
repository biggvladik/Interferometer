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
            section2_width: 390.0,
            section3_width: 195.0,
            
            // Ширина нижней центральной панели (теперь независимая!)
            bottom_width: 795.0,     // Можно задать любую ширину
            
            // Данные для левой панели
            positioner_name: String::from("Positioner_001"),  // Начальное значение
            axis_name: String::from("TrS"),                   // Название оси

            current_value: String::from("0.0"),
            destination_value: String::from("0.0"),
            is_moving: false,
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
                                            self.is_moving = true;
                                            println!("ОСТАНОВКА ДВИЖЕНИЯ");
                                        }
                                    });
                                });
                            });
                            
                            // Секция 2
                            let sec2_response = ui.allocate_response(
                                egui::vec2(self.section2_width, self.top_height),
                                egui::Sense::hover()
                            );
                            
                            ui.painter().rect_stroke(
                                sec2_response.rect,
                                5.0,
                                egui::Stroke::new(2.0, egui::Color32::DARK_GREEN)
                            );
                            
                            ui.allocate_ui_at_rect(sec2_response.rect, |ui| {
                                ui.vertical_centered(|ui| {
                                    ui.heading("Секция 2");
                                    ui.separator();
                                    ui.label("5.2.4 Скорость");
                                    ui.label("5.2.5 Статус");
                                    ui.label("25°C | 24V");
                                });
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