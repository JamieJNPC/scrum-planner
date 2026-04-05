use std::ops::{Add};
use chrono::prelude::*;
use egui::{Response, Sense, Ui, Widget};
use crate::app::model::pi::Sprint;

#[derive(serde::Deserialize, serde::Serialize, Debug, Clone)]
pub struct Member {
    pub name: String,
    pub role: Role,
    pub capacity: f64,
}

impl PartialEq for Member {
    fn eq(&self, other: &Self) -> bool {
        self.name.eq(&other.name)
    }
}
impl Widget for Member {
    fn ui(self, ui: &mut egui::Ui) -> egui::Response {
        ui.vertical(|ui| {
            ui.horizontal(|ui| {
                ui.label("Name");
                ui.label(&self.name);
            });
            ui.horizontal(|ui| {
                ui.label("Role");
                ui.add(self.role);
            });
            ui.horizontal(|ui| {
                ui.label("Capacity");
                ui.label(self.capacity.to_string());
            });
        }).response
    }
}

impl Member {
    pub fn new(name: String, role: Role, capacity: f64) -> Self {
        Member{name, role, capacity}
    }
}

#[derive(serde::Deserialize, serde::Serialize, Clone, Debug, PartialEq)]
pub enum RenderMode {
    OneLine,
    Full,
}

#[derive(serde::Deserialize, serde::Serialize, Clone, Debug)]
pub struct Story {
    pub name: String,
    pub story_points: f64,
    pub description: String,
    pub render_mode: RenderMode,
    pub sprint: Sprint
}

impl PartialEq for Story {
    fn eq(&self, other: &Self) -> bool {
        self.name.eq(&other.name)
    }
}

impl Story {
    pub fn new(name: String, story_points: f64, description: String, sprint: Sprint) -> Self {
        Story{name, story_points, description, render_mode: RenderMode::Full, sprint}
    }
}

impl Widget for Story {
    fn ui(self, ui: &mut egui::Ui) -> Response {
        ui.vertical(|ui| {
            ui.horizontal(|ui| {
                ui.label("Name");
                ui.label(&self.name);
            });
            ui.horizontal(|ui| {
                ui.label("Story Points");
                ui.label(self.story_points.to_string());
            });
            ui.horizontal(|ui| {
                ui.label("Description");
                ui.label(&self.description);
            })
        }).response
    }
}

impl Widget for &Story {
    fn ui(self, ui: &mut egui::Ui) -> Response {
        ui.vertical(|ui| {
            ui.horizontal(|ui| {
                ui.label("Name");
                ui.label(&self.name);
            });
            ui.horizontal(|ui| {
                ui.label("Story Points");
                ui.label(self.story_points.to_string());
            });
            ui.horizontal(|ui| {
                ui.label("Description");
                ui.label(&self.description);
            })
        }).response
    }
}

#[derive(serde::Deserialize, serde::Serialize, Clone, Debug)]
pub struct Objective {
    pub title: String,
    pub stories: Vec<Story>,
    pub render_mode: RenderMode,
}

impl Objective {
    pub fn new(title: String) -> Self {
        Objective{title, stories: vec![], render_mode: RenderMode::Full}
    }

    pub fn add_story(&mut self, story: Story) {
        self.stories.push(story);
    }
}

impl PartialEq for Objective {
    fn eq(&self, other: &Self) -> bool {
        self.title.eq(&other.title)
    }
}
impl Widget for Objective {
    fn ui(mut self, ui: &mut Ui) -> Response {
        ui.vertical(|ui| {
            ui.horizontal(|ui| {
                ui.label("Title");
                ui.label(&self.title);
            });
            //if(self.render_mode == RenderMode::Full) {
                ui.horizontal(|ui| {
                    for story in self.stories.clone() {
                        ui.vertical(|ui| {
                            if ui.button("Delete Story").clicked() {
                                self.stories.retain(|story2: &Story| !story.eq(story2))
                            }
                            ui.add(story.clone());
                        });
                    }
                });
            //}
        }).response
    }
}

#[derive(serde::Deserialize, serde::Serialize, Clone, Debug)]
pub struct Feature {
    pub(crate) name: String,
    pub(crate) objectives: Vec<Objective>,
    pub render_mode: RenderMode
}

impl Feature {
    pub fn new(name: String) -> Self {
        Feature {name, objectives: vec![], render_mode: RenderMode::Full}
    }

    pub fn get_title(&self) -> String {
        self.name.clone()
    }

    pub fn add_objective(&mut self, objective: Objective) {
        self.objectives.push(objective);
    }

    pub fn get_objective(&self, name: &str) -> Option<&Objective> {
        for objective in &self.objectives {
            if objective.title.eq(name) {
                return Some(objective);
            }
        }
        return None;
    }

    pub fn add_story_to_objective(&mut self, objective_name: &String, story: Story) {
        for objective in self.objectives.iter_mut() {
            if objective_name.eq(objective_name) {
                objective.add_story(story);
                return;
            }
        }
    }
}

impl Widget for Feature {

    fn ui(self, ui: &mut Ui) -> Response {
        ui.vertical(|ui| {
            ui.horizontal(|ui| {
                ui.label("Name");
                ui.label(self.name);
            });
            if (self.render_mode == RenderMode::Full) {
                ui.label("Objectives");
                for objective in self.objectives {
                    //ui.label(objective.title.clone());
                    ui.add(objective.clone());
                }
            }
        }).response.interact(Sense::click())
    }
}

impl PartialEq for Feature {
    fn eq(&self, other: &Self) -> bool {
        self.name.eq(&other.name)
    }
}

#[derive(serde::Deserialize, serde::Serialize, Clone, Debug, PartialEq)]
pub(crate) struct Day {
    pub date: NaiveDate,
    pub morning_off: Vec<Member>,
    pub afternoon_off: Vec<Member>,
}

impl Day {
    pub fn new(date: NaiveDate) -> Self {
        Day{date, morning_off: vec![], afternoon_off: vec![]}
    }
}
/// Defines what percentage of a story point a man day is worth for a given role
#[derive(serde::Deserialize, serde::Serialize, Debug)]
#[derive(Clone)]
pub struct Role {
    pub name: String,
    pub velocity: f64
}

impl PartialEq for Role {
    fn eq(&self, other: &Self) -> bool {
        self.name.eq(&other.name)
    }
}

impl Role {
    pub fn new(name: String, velocity: f64) -> Self {
        Role{name, velocity}
    }
}

impl Widget for Role {
    fn ui(self, ui: &mut Ui) -> egui::Response {
        ui.horizontal(|ui| {
            ui.horizontal(|ui| {
                ui.label("Name");
                ui.label(&self.name);
            });
            ui.horizontal(|ui| {
                ui.label("Velocity");
                ui.label(&self.velocity.to_string());
            })
        }).response
    }
}