use eframe::egui;

pub trait PanelComponent {
    fn render(&mut self, ui: &mut egui::Ui, controller: &mut crate::controller::AppController);
}

// Добавляем blanket implementation для всех типов, реализующих PanelComponent
impl<T: PanelComponent> PanelComponent for &mut T {
    fn render(&mut self, ui: &mut egui::Ui, controller: &mut crate::controller::AppController) {
        (**self).render(ui, controller);
    }
}