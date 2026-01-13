use eframe::egui;
use crate::controller::AppController;

use super::{
    PanelComponent,
    LeftPanel, CenterTopPanel, CenterBottomPanel, 
    RightPanel, GraphPanel
};

pub struct AppView {
    left_panel: LeftPanel,
    center_top_panel: CenterTopPanel,
    center_bottom_panel: CenterBottomPanel,
    right_panel: RightPanel,
    graph_panel: GraphPanel,
}

impl AppView {
    pub fn new() -> Self {
        Self {
            left_panel: LeftPanel,
            center_top_panel: CenterTopPanel,
            center_bottom_panel: CenterBottomPanel,
            right_panel: RightPanel,
            graph_panel: GraphPanel,
        }
    }
    
    pub fn render(&mut self, ctx: &egui::Context, controller: &mut AppController) {
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.vertical(|ui| {
                // ВЕРХНЯЯ ЧАСТЬ С ПАНЕЛЯМИ
                egui::ScrollArea::horizontal().show(ui, |ui| {
                    ui.horizontal(|ui| {
                        // Левая панель
                        PanelComponent::render(&mut self.left_panel, ui, controller);
                        
                        ui.add_space(1.0);
                        
                        // Центральная область
                        ui.vertical(|ui| {
                            // Верхняя часть
                            PanelComponent::render(&mut self.center_top_panel, ui, controller);
                            
                            ui.add_space(5.0);
                            
                            // Нижняя часть
                            PanelComponent::render(&mut self.center_bottom_panel, ui, controller);
                        });
                        
                        ui.add_space(5.0);
                        
                        // Правая панель
                        PanelComponent::render(&mut self.right_panel, ui, controller);
                    });
                });
                
                // Отступ между панелями и графиком
                ui.add_space(10.0);
                
                // График
                PanelComponent::render(&mut self.graph_panel, ui, controller);
            });
        });
    }
}