// src/model.rs
use std::collections::VecDeque;

#[derive(Debug, Clone)]
pub struct PositionerModel {
    // Настройки размеров интерфейса
    pub panel_width: f32,
    pub panel_height: f32,
    pub top_height: f32,
    pub bottom_height: f32,
    pub graph_height: f32,
    
    pub section1_width: f32,
    pub section2_width: f32,
    pub section3_width: f32,
    pub bottom_width: f32,
    
    // Данные позиционера
    pub positioner_name: String,
    pub axis_name: String,
    
    // Управление движением
    pub current_value: String,
    pub destination_value: String,
    pub is_moving: bool,
    
    // Настройки шага и скорости
    pub step_value: String,
    pub step_index: usize,
    pub speed_value: String,
    
    // Мониторинг
    pub temperature_value: String,
    pub voltage_value: String,
    
    // Правая панель
    pub repeat_value: String,
    
    // Слайдер
    pub slider_min: f32,
    pub slider_max: f32,
    pub slider_current: f32,
    pub slider_destination: f32,
    
    // График (только данные, без отрисовки)
    pub graph_data: VecDeque<f32>,
    pub max_graph_points: usize,
}

impl Default for PositionerModel {
    fn default() -> Self {
        let mut graph_data = VecDeque::new();
        for i in 0..100 {
            let x = i as f32 / 10.0;
            graph_data.push_back((x.sin() + 1.0) * 500.0);
        }
        
        Self {
            // Размеры
            panel_width: 120.0,
            panel_height: 200.0,
            top_height: 80.0,
            bottom_height: 105.0,
            graph_height: 300.0,
            
            section1_width: 195.0,
            section2_width: 260.0,
            section3_width: 150.0,
            bottom_width: 684.0,
            
            // Данные
            positioner_name: "Positioner_001".to_string(),
            axis_name: "TrS".to_string(),
            
            current_value: "1250.0".to_string(),
            destination_value: "1500.0".to_string(),
            is_moving: false,
            
            step_value: "1/4".to_string(),
            step_index: 1,
            speed_value: "1.0".to_string(),
            
            temperature_value: "25°C".to_string(),
            voltage_value: "24V".to_string(),
            
            repeat_value: "1".to_string(),
            
            slider_min: 0.0,
            slider_max: 2000.0,
            slider_current: 1250.0,
            slider_destination: 1500.0,
            
            graph_data,
            max_graph_points: 200,
        }
    }
}

impl PositionerModel {
    pub fn new() -> Self {
        Self::default()
    }
    
    pub fn update_current_from_string(&mut self) {
        if let Ok(value) = self.current_value.parse::<f32>() {
            self.slider_current = value.clamp(self.slider_min, self.slider_max);
            self.current_value = format!("{:.1}", self.slider_current);
        }
    }
    
    pub fn update_destination_from_string(&mut self) {
        if let Ok(value) = self.destination_value.parse::<f32>() {
            self.slider_destination = value.clamp(self.slider_min, self.slider_max);
            self.destination_value = format!("{:.1}", self.slider_destination);
        }
    }
    
    pub fn update_graph_data(&mut self) {
        // Метод оставляем, но он только обновляет данные
        let new_value = self.slider_current + (rand::random::<f32>() - 0.5) * 100.0;
        self.graph_data.push_back(new_value);
        
        if self.graph_data.len() > self.max_graph_points {
            self.graph_data.pop_front();
        }
    }
    
    pub fn start_movement(&mut self) {
        self.is_moving = true;
        println!("Начато движение от {} к {}", 
                self.slider_current, self.slider_destination);
    }
    
    pub fn stop_movement(&mut self) {
        self.is_moving = false;
        println!("ОСТАНОВКА ДВИЖЕНИЯ");
    }
    
    pub fn disconnect(&mut self) {
        println!("Попытка отключения от позиционера...");
    }
}