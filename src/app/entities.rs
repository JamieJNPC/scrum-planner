use std::ops::{Add, AddAssign};
use chrono::prelude::*;
use egui::{Ui, Widget};

pub struct Capacity {
    corrective_maintenance: f64,
    evolutive_maintenance: f64,
    support: f64,
    chm: f64,
}

impl Capacity {
    pub fn new(corrective_maintenance: f64, evolutive_maintenance: f64, support: f64, chm: f64) -> Self {
        Capacity{corrective_maintenance, evolutive_maintenance, support, chm}
    }

    fn get_feature_capacity(&self) -> f64 {
        100.0 - self.corrective_maintenance - self.evolutive_maintenance - self.support - self.chm
    }
}

impl AddAssign for Capacity {
    fn add_assign(&mut self, other: Self) {
        Capacity {
            corrective_maintenance: self.corrective_maintenance + other.corrective_maintenance,
            evolutive_maintenance: self.evolutive_maintenance + other.evolutive_maintenance,
            support: self.support + other.support,
            chm: self.chm + other.chm,
        };
    }
}
impl Add for Capacity {
    type Output = Capacity;
    fn add(self, other: Self) -> Capacity {
        Capacity {
            corrective_maintenance: self.corrective_maintenance + other.corrective_maintenance,
            evolutive_maintenance: self.evolutive_maintenance + other.evolutive_maintenance,
            support: self.support + other.support,
            chm: self.chm + other.chm,
        }
    }
}

#[derive(serde::Deserialize, serde::Serialize, Debug)]
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
            })
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

struct Story {
    name: String,
    story_points: f64,
    description: String,
}

struct Objective {
    name: String
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

/// Calculates the total allocation for each capacity for an entire sprint
fn calculate_capacities(members: &Vec<Member>, sprint: &Sprint, cm: f64,
                        em: f64, support: f64, chm: f64) -> Capacity {
    let mut res = Capacity::new(0.0,0.0,0.0,0.0);
    for day in &sprint.days {
        res += calculate_capacity_for_day(members, &day, cm, em, support, chm);
    }
    res
}

/// Calculates the total allocation for each capacity for a given day
/// param
fn calculate_capacity_for_day(members: &Vec<Member>, day: &Day, cm: f64,
                              em: f64, support: f64, chm: f64) -> Capacity {
    let mut res = Capacity::new(0.0,0.0,0.0,0.0);
    for member in members.iter() {
        res += calculate_capacity_for_member(member, day, cm, em, support, chm);
    }
    res
}

fn calculate_capacity_for_member(member: &Member, day: &Day, cm: f64,
                                 em: f64, support: f64, chm: f64) -> Capacity {
    let mut multiplier = 0.0;
    if !day.morning_off.contains(member) {
        multiplier += 0.5;
    }
    if !day.afternoon_off.contains(member) {
        multiplier += 0.5;
    }
    let velocity = member.capacity * multiplier * member.role.velocity;
    Capacity::new(cm * velocity, em * velocity, support * velocity, chm * velocity)
}