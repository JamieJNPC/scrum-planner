use egui::{Response, Ui, Widget};
use crate::app::model::capacity::{Capacities, Capacity};

#[derive(serde::Deserialize, serde::Serialize, Debug, Clone)]
pub struct CapacityCreationWindow {
    name: String,
    capacities: Vec<(String, String)>,
    result: Option<Capacities>,
    
}

impl CapacityCreationWindow {
    pub fn new() -> Self {
        CapacityCreationWindow {name: String::new(), capacities: Vec::new(), result: None}
    }
}

impl Widget for CapacityCreationWindow {
    fn ui(mut self, ui: &mut Ui) -> Response {
        ui.vertical(|ui| {
            ui.horizontal(|ui| {
                ui.label("Name");
                ui.text_edit_singleline(&mut self.name);
            });
            ui.horizontal(|ui| {
                ui.label("capacity");
                ui.label("allocation")
            });
            for (name, capacity) in self.capacities.iter_mut() {
                ui.horizontal(|ui| {
                    ui.text_edit_singleline(name);
                    ui.text_edit_singleline(capacity);
                });
            }
            ui.horizontal(|ui| {
                if ui.button("Add capacity").clicked() {
                    self.capacities.push((String::new(), String::new()));
                }
                if ui.button("Create").clicked() {
                    let mut capacities: Vec<Capacity> = vec![];
                    for (name, capacity) in self.capacities.iter() {
                        capacities.push(Capacity::new(name.clone(), capacity.parse().unwrap()));
                    }
                    self.result = Some(Capacities::new(self.name, capacities));
                }
            })
        }).response
    }
}