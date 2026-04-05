use std::ops::{Add, AddAssign};
use crate::app::entities::{Day, Member};
use crate::app::model::pi::Sprint;

#[derive(serde::Deserialize, serde::Serialize, Debug, Clone)]
pub struct Capacity {
    pub label: String,
    pub capacity: f64,
}

impl Capacity {
    pub fn new(label: String, capacity: f64) -> Self {
        Capacity { label, capacity }
    }
}

#[derive(serde::Deserialize, serde::Serialize, Debug, Clone)]
pub struct Capacities {
    name: String,
    capacities: Vec<Capacity>,
}

impl Capacities {
    pub fn new(name: String, capacities: Vec<Capacity>) -> Self {
        Capacities {name, capacities}
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
        self
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