#[derive(Clone)]
pub struct PositionerData {
    // Общие данные
    pub positioner_name: String,
    pub axis_name: String,
    
    // Данные для движения
    pub current_value: String,
    pub destination_value: String,
    pub is_moving: bool,
    
    // Настройки
    pub step_value: String,
    pub step_index: usize,
    pub speed_value: String,
    
    // Мониторинг
    pub temperature_value: String,
    pub voltage_value: String,
    
    // Повторение
    pub repeat_value: String,
    
    // Данные для слайдера
    pub slider_min: f32,
    pub slider_max: f32,
    pub slider_current: f32,
    pub slider_destination: f32,
    
    // Данные для графика
    pub graph_data: Vec<f32>,
}

#[derive(Clone)]
pub struct AppConfig {
    // Все размеры интерфейса
    pub panel_width: f32,
    pub panel_height: f32,
    pub top_height: f32,
    pub bottom_height: f32,
    pub graph_height: f32,
    
    // Размеры трех секций верхней центральной области
    pub section1_width: f32,
    pub section2_width: f32,
    pub section3_width: f32,
    
    // Ширина нижней центральной панели
    pub bottom_width: f32,
    pub graph_width: f32,
}

impl Default for PositionerData {
    fn default() -> Self {
        let mut graph_data = Vec::new();
        for i in 0..100 {
            let x = i as f32 / 10.0;
            graph_data.push((x.sin() + 1.0) * 500.0);
        }
        
        Self {
            positioner_name: String::from("Positioner_001"),
            axis_name: String::from("TrS"),
            current_value: String::from("1250.0"),
            destination_value: String::from("1500.0"),
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
        }
    }
}

impl Default for AppConfig {
    fn default() -> Self {
        Self {
            panel_width: 120.0,
            panel_height: 200.0,
            top_height: 80.0,
            bottom_height: 105.0,
            graph_height: 300.0,
            section1_width: 195.0,
            section2_width: 260.0,
            section3_width: 150.0,
            bottom_width: 684.0,
            graph_width: 810.0,
        }
    }
}