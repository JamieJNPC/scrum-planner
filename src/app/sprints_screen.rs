use crate::MainApp;
use egui::{Context, Frame};

impl MainApp {
    pub(crate) fn render_sprints_screen(&mut self, ctx: &Context, ui: &mut egui::Ui) {
        ui.heading("PIs");
        ui.separator();
        ui.horizontal(|ui| {
            for mut pi in self.main_app_data.pis.clone() {
                pi.add_stories_for_sprints(&mut self.main_app_data.features);
                ui.add(pi);
            }
        });
    }
}