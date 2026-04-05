use std::ops::Add;
use chrono::{NaiveDate, Utc};
use egui::{Response, Ui, Widget};
use crate::app::entities::{Feature, Story};
use crate::app::window_data::PiOptions;

#[derive(serde::Deserialize, serde::Serialize, Clone, Debug)]
#[derive(PartialEq)]
pub struct PI {
    pub name: String,
    pub sprints: Vec<Sprint>,
}

impl PI {

    pub fn new2() -> Self {
        PI {name: String::new(), sprints: Vec::new() }
    }

    pub fn create(pi: &PiOptions) -> Self {
        Self::new(&pi.title, &pi.start_date.get_date(), &pi.number_of_sprints.parse().unwrap(), &pi.weeks_in_sprint.parse().unwrap())
    }
    pub fn new(name: &String, start_date: &NaiveDate, number_of_sprints: &i32, weeks_in_sprint: &i32) -> Self {
        let mut sprints = Vec::new();
        let days_in_sprint = weeks_in_sprint * 7;
        for i in 0..number_of_sprints.clone() {
            let sprint_start = start_date.add(chrono::Duration::days((days_in_sprint * i) as i64));
            let sprint_end = sprint_start.add(chrono::Duration::days(days_in_sprint as i64));
            let sprint_name: String = name.to_owned() + "." + i.to_string().as_str();
            sprints.push(Sprint::new(&sprint_name, sprint_start, sprint_end));
        }
        PI {name: name.clone(), sprints }
    }

    pub(crate) fn add_stories_for_sprints(&mut self, features: &mut Vec<Feature>) {
        for mut sprint in self.sprints.iter_mut() {
            for feature in features.iter_mut() {
                for objective in feature.objectives.clone() {
                    for story in objective.stories.clone() {
                        if story.sprint.eq(&sprint) && !sprint.stories.contains(&story) {
                            sprint.stories.push(story);
                        } else if !story.sprint.eq(&sprint) && sprint.stories.contains(&story) {
                            sprint.remove_story(&story);
                        }
                    }
                }
            }
        }
    }
}

impl Widget for PI {
    fn ui(self, ui: &mut Ui) -> Response {
        ui.vertical(|ui| {
            ui.heading(self.name);
            for sprint in self.sprints {
                ui.add(sprint);
            }
        }).response
    }
}

#[derive(serde::Deserialize, serde::Serialize, Clone, Debug, PartialEq)]
pub struct Sprint {
    pub(crate) name: String,
    pub days: Vec<crate::app::entities::Day>,
    pub stories: Vec<Story>,
    pub start_date: NaiveDate,
    pub end_date: NaiveDate,
}

impl Sprint {

    pub fn new2() -> Self {
        Sprint {name: String::new(), days: vec![], stories: vec![], start_date: Utc::now().date_naive(), end_date: Utc::now().date_naive()}
    }
    fn new(name: &String, start_date: NaiveDate, end_date: NaiveDate) -> Self {
        let mut date_counter = start_date.clone();
        let mut day_res: Vec<crate::app::entities::Day> = Vec::new();
        while date_counter < end_date {
            date_counter = date_counter.add(chrono::Duration::days(1));
            day_res.push(crate::app::entities::Day::new(date_counter));
        }
        Sprint {name: name.clone(), days: day_res, stories: vec![], start_date, end_date}
    }

    pub fn remove_story(&mut self, story: &Story) {
        for i in 0..self.stories.len() {
            if self.stories.get(i).unwrap().eq(story) {
                self.stories.remove(i);
            }
        }
    }
}

impl Widget for Sprint {
    fn ui(self, ui: &mut Ui) -> Response {
        ui.vertical(|ui| {
            ui.heading("Sprint");
            ui.label(self.start_date.to_string() + " - " + &*self.end_date.to_string());
            for story in self.stories {
                ui.add(story);
            }
        }).response
    }
}