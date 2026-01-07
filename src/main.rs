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
}

impl FixedPanelsApp {
    fn new() -> Self {
        Self {
            // Все размеры задаются здесь в одном месте
            panel_width: 200.0,      // Ширина боковых панелей
            panel_height: 210.0,     // Высота боковых панелей
            top_height: 100.0,       // Высота верхней центральной области
            bottom_height: 100.0,    // Высота нижней центральной области
            
            // Ширины трех секций верхней центральной области
            section1_width: 190.0,
            section2_width: 400.0,
            section3_width: 190.0,
            
            // Ширина нижней центральной панели (теперь независимая!)
            bottom_width: 795.0,     // Можно задать любую ширину
        }
    }
    
    // Вычисляемые свойства
    fn center_upper_width(&self) -> f32 {
        // Сумма ширин верхних секций
        self.section1_width + self.section2_width + self.section3_width
    }
    
    fn total_width(&self) -> f32 {
        // Общая ширина всех элементов (примерно)
        self.panel_width * 2.0 + self.center_upper_width().max(self.bottom_width) + 20.0
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
            // Используем ScrollArea для горизонтальной прокрутки при необходимости
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
                        5.0,
                        egui::Stroke::new(2.0, egui::Color32::BLUE)
                    );
                    
                    // Содержимое левой панели
                    ui.allocate_ui_at_rect(left_response.rect, |ui| {
                        ui.vertical_centered(|ui| {
                            ui.heading("Левая панель");
                            ui.separator();
                            ui.label(format!("{} × {} px", self.panel_width, self.panel_height));
                            ui.separator();
                            ui.label("Настройки");
                            ui.label("Конфигурация");
                            ui.label("Статус");
                        });
                    });
                    
                    // Небольшой отступ между панелями
                    ui.add_space(5.0);
                    
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
                                5.0,
                                egui::Stroke::new(2.0, egui::Color32::DARK_GREEN)
                            );
                            
                            ui.allocate_ui_at_rect(sec1_response.rect, |ui| {
                                ui.vertical_centered(|ui| {
                                    ui.heading("Секция 1");
                                    ui.separator();
                                    ui.label("5.2.2 Позиционеры");
                                    ui.label("5.2.3 Текущая позиция");
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
            
            ui.separator();
            
            // ИНФОРМАЦИЯ О РАЗМЕРАХ
            ui.vertical(|ui| {
                ui.heading("📏 Все размеры задаются независимо в методе new():");
                
                ui.horizontal(|ui| {
                    ui.vertical(|ui| {
                        ui.label("Боковые панели:");
                        ui.monospace(format!("Ширина: {} px", self.panel_width));
                        ui.monospace(format!("Высота: {} px", self.panel_height));
                    });
                    
                    ui.add_space(20.0);
                    
                    ui.vertical(|ui| {
                        ui.label("Центральная область (верх):");
                        ui.monospace(format!("Секция 1: {} px", self.section1_width));
                        ui.monospace(format!("Секция 2: {} px", self.section2_width));
                        ui.monospace(format!("Секция 3: {} px", self.section3_width));
                        ui.monospace(format!("Сумма: {} px", self.center_upper_width()));
                        ui.monospace(format!("Высота: {} px", self.top_height));
                    });
                    
                    ui.add_space(20.0);
                    
                    ui.vertical(|ui| {
                        ui.label("Центральная область (низ):");
                        ui.monospace(format!("Ширина: {} px", self.bottom_width));
                        ui.monospace(format!("Высота: {} px", self.bottom_height));
                    });
                });
                
                ui.separator();
                
                // Сравнение размеров (информационно)
                ui.label("📊 Сравнение размеров:");
                if (self.center_upper_width() - self.bottom_width).abs() < 0.1 {
                    ui.colored_label(egui::Color32::GREEN, 
                        format!("✓ Ширина верхней области ({} px) равна ширине нижней области ({} px)", 
                            self.center_upper_width(), self.bottom_width));
                } else if self.center_upper_width() > self.bottom_width {
                    ui.colored_label(egui::Color32::YELLOW, 
                        format!("⚠ Верхняя область шире: {} px > {} px (разница: {} px)", 
                            self.center_upper_width(), self.bottom_width,
                            self.center_upper_width() - self.bottom_width));
                } else {
                    ui.colored_label(egui::Color32::YELLOW, 
                        format!("⚠ Нижняя область шире: {} px > {} px (разница: {} px)", 
                            self.bottom_width, self.center_upper_width(),
                            self.bottom_width - self.center_upper_width()));
                }
                
                ui.label(format!("Общая примерная ширина окна: {} px", self.total_width()));
            });
        });
    }
}