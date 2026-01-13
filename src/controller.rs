use crate::model::{PositionerData, AppConfig};
use eframe::egui;

pub struct AppController {
    pub model: PositionerData,
    pub config: AppConfig,
}

impl AppController {
    pub fn new() -> Self {
        Self {
            model: PositionerData::default(),
            config: AppConfig::default(),
        }
    }
    
    pub fn update_current_from_string(&mut self) {
        if let Ok(value) = self.model.current_value.parse::<f32>() {
            self.model.slider_current = value.clamp(self.model.slider_min, self.model.slider_max);
            self.model.current_value = format!("{:.1}", self.model.slider_current);
        }
    }
    
    pub fn update_destination_from_string(&mut self) {
        if let Ok(value) = self.model.destination_value.parse::<f32>() {
            self.model.slider_destination = value.clamp(self.model.slider_min, self.model.slider_max);
            self.model.destination_value = format!("{:.1}", self.model.slider_destination);
        }
    }
    
    pub fn update_graph_data(&mut self) {
        let new_value = self.model.slider_current + (rand::random::<f32>() - 0.5) * 100.0;
        self.model.graph_data.push(new_value);
        
        if self.model.graph_data.len() > 200 {
            self.model.graph_data.remove(0);
        }
    }
    
    pub fn handle_start_movement(&mut self) {
        self.model.is_moving = true;
        println!("Начато движение от {} к {}", 
                self.model.slider_current, self.model.slider_destination);
    }
    
    pub fn handle_stop_movement(&mut self) {
        self.model.is_moving = false;
        println!("ОСТАНОВКА ДВИЖЕНИЯ");
    }
    
    pub fn handle_disconnect(&mut self) {
        println!("Попытка отключения от позиционера...");
    }
    
    pub fn handle_slider_drag(&mut self, pos_x: f32, slider_left: f32, slider_width: f32) {
        let x_to_value = |x: f32| -> f32 {
            let normalized = (x - slider_left) / slider_width;
            self.model.slider_min + normalized * (self.model.slider_max - self.model.slider_min)
        };
        
        let value_to_x = |value: f32| -> f32 {
            let normalized = (value - self.model.slider_min) / (self.model.slider_max - self.model.slider_min);
            slider_left + normalized * slider_width
        };
        
        let current_x = value_to_x(self.model.slider_current);
        let dest_x = value_to_x(self.model.slider_destination);
        
        let dist_to_current = (pos_x - current_x).abs();
        let dist_to_dest = (pos_x - dest_x).abs();
        
        let new_value = x_to_value(pos_x);
        
        if dist_to_current < dist_to_dest {
            self.model.slider_current = new_value.clamp(self.model.slider_min, self.model.slider_max);
            self.model.current_value = format!("{:.1}", self.model.slider_current);
        } else {
            self.model.slider_destination = new_value.clamp(self.model.slider_min, self.model.slider_max);
            self.model.destination_value = format!("{:.1}", self.model.slider_destination);
        }
    }
}

impl Default for AppController {
    fn default() -> Self {
        Self::new()
    }
}

impl eframe::App for AppController {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        if ctx.input(|i| i.key_pressed(egui::Key::Enter)) {
            self.update_current_from_string();
            self.update_destination_from_string();
        }
        
        self.update_graph_data();
        
        // Создаем View напрямую здесь
        use crate::view::AppView;
        let mut view = AppView::new();
        view.render(ctx, self);
    }
}