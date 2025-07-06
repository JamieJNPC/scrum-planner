use std::ops::{Add, AddAssign};
use chrono::prelude::*;
use egui::{Response, Ui, Widget};

#[derive(Clone)]
pub struct Capacity {
    pub label: String,
    pub capacity: f64,
}

#[derive(Clone)]
pub struct Capacities {
    capacities: Vec<Capacity>,
}

impl Capacities {
    pub fn new() -> Self {
        Capacities {capacities: vec![]}
    }

    fn get_capacity_by_name(&self, name: &str) -> Option<&Capacity> {
        self.capacities.iter().find(|c| c.label == name)
    }

    fn get_feature_capacity(&self) -> Capacity {
        self.get_capacity_by_name("Feature").unwrap_or(&Capacity {
            label: "Feature".to_string(),
            capacity: 0.0
        }).clone()
    }
}

impl AddAssign for Capacities {
    fn add_assign(&mut self, other: Self) {
        for capacity_1 in other.capacities {
            if self.capacities.iter().find(|c| c.label == capacity_1.label).is_none() {
                self.capacities.push(capacity_1);
            } else {
                for capacity in self.capacities.iter_mut() {
                    if capacity.label == capacity_1.label {
                        capacity.capacity += capacity_1.capacity;
                    }
                }
            }
        }
    }
}
impl Add for Capacities {
    type Output = Capacities;
    fn add(mut self, other: Self) -> Capacities {
        for capacity_1 in other.capacities {
            if self.capacities.iter().find(|c| c.label == capacity_1.label).is_none() {
                self.capacities.push(capacity_1);
            } else {
                for capacity in self.capacities.iter_mut() {
                    if capacity.label == capacity_1.label {
                        capacity.capacity += capacity_1.capacity;
                    }
                }
            }
        };
        return self;
    }
}

#[derive(serde::Deserialize, serde::Serialize, Debug, Clone)]
pub struct Member {
    name: String,
    role: Role,
    capacity: f64,
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

struct PI {
    sprints: Vec<Sprint>
}

struct Sprint {
    days: Vec<Day>,
    stories: Vec<Story>
}

#[derive(serde::Deserialize, serde::Serialize, Clone, Debug)]
pub struct Story {
    pub name: String,
    pub story_points: f64,
    pub description: String,
}

impl Story {
    pub fn new(name: String, story_points: f64, description: String) -> Self {
        Story{name, story_points, description}
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

#[derive(serde::Deserialize, serde::Serialize, Clone, Debug)]
pub struct Objective {
    pub title: String,
    pub stories: Vec<Story>
}

impl Objective {
    pub fn new(title: String) -> Self {
        Objective{title, stories: vec![]}
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
    fn ui(self, ui: &mut Ui) -> Response {
        ui.vertical(|ui| {
            ui.horizontal(|ui| {
                ui.label("Title");
                ui.label(&self.title);
            });
            for story in &self.stories {
                ui.add(story.clone());
            }
        }).response
    }
}

#[derive(serde::Deserialize, serde::Serialize, Clone, Debug)]
pub struct Feature {
    pub(crate) name: String,
    pub(crate) objectives: Vec<Objective>,
}

impl Feature {
    pub fn new(name: String) -> Self {
        Feature {name, objectives: vec![]}
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
}

impl Widget for Feature {

    fn ui(self, ui: &mut Ui) -> Response {
        ui.vertical(|ui| {
            ui.horizontal(|ui| {
                ui.label("Name");
                ui.label(self.name);
            });
            for objective in self.objectives {
                //ui.label(objective.title.clone());
                ui.add(objective.clone());
            }
        }).response
    }
}

impl PartialEq for Feature {
    fn eq(&self, other: &Self) -> bool {
        self.name.eq(&other.name)
    }
}

struct Day {
    date: NaiveDate,
    morning_off: Vec<Member>,
    afternoon_off: Vec<Member>,
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
/*
/// Calculates the total allocation for each capacity for an entire sprint
fn calculate_capacities(members: &Vec<Member>, sprint: &Sprint, cm: f64,
                        em: f64, support: f64, chm: f64) -> Capacities {
    let mut res = Capacities::new(0.0, 0.0, 0.0, 0.0);
    for day in &sprint.days {
        res += calculate_capacity_for_day(members, &day, cm, em, support, chm);
    }
    res
}

/// Calculates the total allocation for each capacity for a given day
/// param
fn calculate_capacity_for_day(members: &Vec<Member>, day: &Day, cm: f64,
                              em: f64, support: f64, chm: f64) -> Capacities {
    let mut res = Capacities::new(0.0, 0.0, 0.0, 0.0);
    for member in members.iter() {
        res += calculate_capacity_for_member(member, day, cm, em, support, chm);
    }
    res
}

fn calculate_capacity_for_member(member: &Member, day: &Day, cm: f64,
                                 em: f64, support: f64, chm: f64) -> Capacities {
    let mut multiplier = 0.0;
    if !day.morning_off.contains(member) {
        multiplier += 0.5;
    }
    if !day.afternoon_off.contains(member) {
        multiplier += 0.5;
    }
    let velocity = member.capacity * multiplier * member.role.velocity;
    Capacities::new(cm * velocity, em * velocity, support * velocity, chm * velocity)
}
 */